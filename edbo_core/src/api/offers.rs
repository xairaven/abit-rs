use crate::api;
use crate::api::{ApiError, ErrorResponse, INTERVAL_FOR_REQUESTS};
use crate::error::CoreError;
use crate::model::ModelError;
use crate::model::offer::Offer;
use crate::model::offer_type::OfferType;
use crate::model::offers_university::OffersUniversity;
use crate::model::speciality::Speciality;
use crate::model::study_form::StudyForm;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use url::Url;

pub async fn list(
    offers_of_institutes: &mut [OffersUniversity],
) -> Result<Vec<Offer>, CoreError> {
    let base_url = format!("{}/offer/", api::links::MAIN);

    let client = reqwest::Client::builder()
        .build()
        .map_err(ApiError::FailedBuildClient)?;

    let mut ticker = tokio::time::interval(INTERVAL_FOR_REQUESTS);

    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static(api::USER_AGENT));

    let mut offers: Vec<Offer> = vec![];

    // Institute ID, Offer ID
    let mut not_budgetary_offers: Vec<u32> = vec![];
    for university_offers_relation in offers_of_institutes.iter() {
        for offer_id in &university_offers_relation.offers {
            ticker.tick().await;

            let url = Url::parse(&format!("{}{}", base_url, offer_id))
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
            log::debug!("Text from offer response: {:?}", text);

            loop {
                match serde_json::from_str::<ErrorResponse>(&text) {
                    Ok(error) => error.handle_request_limit().await,
                    Err(_) => {
                        log::info!("Offer response success for {} offer ID.", offer_id);
                        break;
                    },
                }
            }

            let offer_type = extract_info_by_tag::<String>("ustn", &text)?;
            let offer_type =
                OfferType::try_from(offer_type.as_str()).map_err(ModelError::from)?;
            if matches!(offer_type, OfferType::NonBudgetary) {
                not_budgetary_offers.push(*offer_id);
                continue;
            }

            let faculty = extract_info_by_tag::<String>("ufn", &text)?;
            let education_program = extract_info_by_tag::<String>("usn", &text)?;
            let master_type = extract_info_by_tag::<String>("mptn", &text)?;
            let speciality = Speciality::try_from(
                extract_info_by_tag::<String>("ssc", &text)?.as_str(),
            )
            .map_err(ModelError::from)?;
            let title = extract_info_by_tag::<String>("spn", &text)?;
            let license_volume = extract_info_by_tag::<i32>("ol", &text)?;
            let study_form = StudyForm::try_from(
                extract_info_by_tag::<String>("efn", &text)?.as_str(),
            )
            .map_err(ModelError::from)?;
            let budgetary_places = if let OfferType::Open = offer_type {
                match extract_info_by_tag::<i32>("ox", &text) {
                    Ok(value) => value,
                    Err(_) => {
                        // ISSUE: https://vstup.edbo.gov.ua/offer/1513669
                        log::error!(
                            "Failed to get budgetary places for OPEN offer ID: {}",
                            offer_id
                        );
                        not_budgetary_offers.push(*offer_id);
                        continue;
                    },
                }
            } else if let OfferType::Fixed = offer_type {
                match extract_info_by_tag::<i32>("ob", &text) {
                    Ok(value) => value,
                    Err(_) => {
                        log::error!(
                            "Failed to get budgetary places for FIXED offer ID: {}",
                            offer_id
                        );
                        not_budgetary_offers.push(*offer_id);
                        continue;
                    },
                }
            } else {
                return Err(ApiError::FailedParsing(text.to_string()).into());
            };

            let offer = Offer {
                id: *offer_id,
                title,
                education_program,
                faculty,
                speciality,
                master_type,
                license_volume,
                study_form,
                budgetary_places,
            };
            offers.push(offer);
        }
    }

    // Removing non-budgetary offers
    'offer_loop: for offer_id in not_budgetary_offers {
        for university_offers_relation in offers_of_institutes.iter_mut() {
            if let Ok(index) = university_offers_relation.offers.binary_search(&offer_id)
            {
                university_offers_relation.offers.remove(index);
                continue 'offer_loop;
            }
        }
    }

    Ok(offers)
}

fn extract_info_by_tag<T: serde::de::DeserializeOwned>(
    tag: &str, text: &str,
) -> Result<T, ApiError> {
    if let Some(script_start) = text.find("let offer") {
        let snippet = text
            .get(script_start..)
            .ok_or(ApiError::FailedParsing(text.to_string()))?;
        let pattern = format!(
            r#""{}"\s*:\s*(?P<val>"(?:[^"\\]|\\.)*"|-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?|true|false|null)"#,
            regex::escape(tag)
        );
        let re = Regex::new(&pattern)?;
        if let Some(captures) = re.captures(snippet) {
            let value = captures
                .name("val")
                .ok_or(ApiError::FailedParsing(text.to_string()))?
                .as_str();
            return match serde_json::from_str::<T>(value) {
                Ok(value) => Ok(value),
                Err(_) => Err(ApiError::FailedParsing(text.to_string())),
            };
        }
    }
    Err(ApiError::FailedParsing(text.to_string()))
}
