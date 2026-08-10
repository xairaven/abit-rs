use crate::api::{ApiError, ErrorResponse, INTERVAL_FOR_REQUESTS};
use crate::error::CoreError;
use crate::model::ModelError;
use crate::model::degree::Degree;
use crate::model::offer::Offer;
use crate::model::offer_type::OfferType;
use crate::model::offers_university::OffersUniversity;
use crate::model::speciality::Speciality;
use crate::model::study_form::StudyForm;
use crate::{api, request};
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use url::Url;

// TODO: Refactor
// Warning: this function has too many lines (115/100)
pub async fn list(
    offers_of_institutes: &[OffersUniversity],
) -> Result<Vec<Offer>, CoreError> {
    let base_url = format!("{}/offer/", api::links::MAIN);
    let amount = amount(offers_of_institutes);
    log::info!("Started parsing offers. Total amount: {amount}");

    let client = request::Client::build()?;

    let mut ticker = tokio::time::interval(INTERVAL_FOR_REQUESTS);

    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static(api::USER_AGENT));

    let mut offers: Vec<Offer> = vec![];

    // Needed for logging purposes (Progress)
    let mut counter: usize = 0;

    // Institute ID, Offer ID
    for university_offers_relation in offers_of_institutes {
        for offer_id in &university_offers_relation.offers {
            ticker.tick().await;

            let url = Url::parse(&format!("{base_url}{offer_id}"))
                .map_err(ApiError::FailedToParseUrl)?;

            let response = client
                .get(url)
                .headers(headers.clone())
                .send()
                .await
                .map_err(ApiError::RequestFailed)?;

            let text = response
                .text()
                .await
                .map_err(ApiError::FailedToGetResponseText)?;
            log::debug!("Text from offer response: {text:?}");

            loop {
                if let Ok(error) = serde_json::from_str::<ErrorResponse>(&text) {
                    error.handle_request_limit().await;
                } else {
                    counter += 1;
                    log::info!(
                        "({counter}/{amount}) Offer response success. ID: {offer_id}.",
                    );
                    break;
                }
            }

            let offer_type = extract_info_by_tag::<String>("ustn", &text)?;
            let offer_type =
                OfferType::try_from(offer_type.as_str()).map_err(ModelError::from)?;
            if matches!(offer_type, OfferType::NonBudgetary) {
                log::warn!("Skipping NON-BUDGETARY offer ID: {offer_id}");
                continue;
            }

            // ISSUE: https://vstup.edbo.gov.ua/offer/1454003
            let faculty = extract_info_by_tag::<String>("ufn", &text).ok();
            let education_program =
                extract_info_by_tag::<String>("usn", &text).unwrap_or_default();
            let master_type = extract_info_by_tag::<String>("mptn", &text).ok();
            let speciality = Speciality::try_from(
                extract_info_by_tag::<String>("ssc", &text)?.as_str(),
            )
            .map_err(ModelError::from)?;
            let title = extract_info_by_tag::<String>("spn", &text)?;
            let Ok(license_volume) = extract_info_by_tag::<i32>("ol", &text) else {
                // ISSUE: https://vstup.edbo.gov.ua/offer/1513669
                log::error!("Failed to get license volume for offer ID: {offer_id}");
                continue;
            };
            let study_form = StudyForm::try_from(
                extract_info_by_tag::<String>("efn", &text)?.as_str(),
            )
            .map_err(ModelError::from)?;
            let budgetary_places = if matches!(offer_type, OfferType::Open) {
                if let Ok(value) = extract_info_by_tag::<i32>("ox", &text) {
                    value
                } else {
                    // ISSUE: https://vstup.edbo.gov.ua/offer/1513669
                    log::error!(
                        "Failed to get budgetary places for OPEN offer ID: {offer_id}"
                    );
                    continue;
                }
            } else if matches!(offer_type, OfferType::Fixed) {
                if let Ok(value) = extract_info_by_tag::<i32>("ob", &text) {
                    value
                } else {
                    log::error!(
                        "Failed to get budgetary places for FIXED offer ID: {offer_id}"
                    );
                    continue;
                }
            } else {
                return Err(ApiError::FailedParsing(text.clone()).into());
            };

            let offer = Offer {
                id: *offer_id,
                title,
                degree: Degree::Master,
                education_program,
                faculty,
                speciality,
                funding_type: offer_type,
                master_type,
                study_form,
                license_volume,
                budgetary_places,
            };
            offers.push(offer);
        }
    }

    Ok(offers)
}

fn extract_info_by_tag<T: serde::de::DeserializeOwned>(
    tag: &str, text: &str,
) -> Result<T, ApiError> {
    if let Some(script_start) = text.find("let offer") {
        let Some(snippet) = text.get(script_start..) else {
            log::error!("Extract info by tag failed for tag: {tag}");
            return Err(ApiError::FailedParsing(text.to_string()));
        };
        let pattern = format!(
            r#""{}"\s*:\s*(?P<val>"(?:[^"\\]|\\.)*"|-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?|true|false|null)"#,
            regex::escape(tag)
        );
        let re = Regex::new(&pattern)?;
        if let Some(captures) = re.captures(snippet) {
            let value = if let Some(v) = captures.name("val") {
                v.as_str()
            } else {
                log::error!("Extract info by tag failed for tag: {tag}");
                return Err(ApiError::FailedParsing(text.to_string()));
            };
            return serde_json::from_str::<T>(value).map_or_else(
                |_| {
                    log::error!("Extract info by tag failed for tag: {tag}");
                    Err(ApiError::FailedParsing(text.to_string()))
                },
                |value| Ok(value),
            );
        }
    }
    log::error!("Extract info by tag failed for tag: {tag}");
    Err(ApiError::FailedParsing(text.to_string()))
}

fn amount(offers_of_institutes: &[OffersUniversity]) -> usize {
    let mut amount: usize = 0;
    for relation in offers_of_institutes {
        amount += relation.offers.len();
    }

    amount
}
