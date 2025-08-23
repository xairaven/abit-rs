use crate::api;
use crate::api::{ApiError, ApiFetcherForm, ErrorResponse, INTERVAL_FOR_REQUESTS};
use crate::dto::application::ApplyRequestDtoMap;
use crate::error::CoreError;
use crate::model::application::Application;
use crate::model::offer::Offer;
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;
use url::Url;

pub async fn list(offers: &[Offer]) -> Result<Vec<Application>, CoreError> {
    let base_url = format!("{}/offer-requests/", api::links::MAIN);
    let base_url = Url::parse(&base_url).map_err(ApiError::FailedToParseUrl)?;

    let amount = offers.len();
    let mut counter: usize = 0;

    log::info!(
        "Started parsing applications. Total amount of offers: {}",
        amount
    );

    let client = reqwest::Client::builder()
        .build()
        .map_err(ApiError::FailedBuildClient)?;

    let mut form: HashMap<String, i32> = HashMap::new();
    form.insert(String::from("last"), 1);

    let mut ticker = tokio::time::interval(INTERVAL_FOR_REQUESTS);

    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static(api::USER_AGENT));
    headers.insert(
        "Referer",
        HeaderValue::from_str(base_url.as_str()).map_err(ApiError::InvalidHeaderValue)?,
    );

    let mut applications: Vec<Application> = vec![];
    for offer in offers {
        let mut parameters = ApplicantsApi {
            id: offer.id,
            last: 0,
        };

        loop {
            ticker.tick().await;

            let form = parameters.create_form();

            let response = client
                .post(base_url.clone())
                .headers(headers.clone())
                .form(&form)
                .send()
                .await
                .map_err(ApiError::RequestFailed)?;

            let text = response
                .text()
                .await
                .map_err(ApiError::FailedToGetResponseText)?;
            log::debug!("Text from response of application requests: {:?}", text);

            if text.is_empty() {
                log::warn!("Zero applicants for offer ID: {}", offer.id);
                continue;
            }

            let dto_map = loop {
                match serde_json::from_str::<ApplyRequestDtoMap>(&text) {
                    Ok(value) => {
                        counter += 1;
                        log::info!(
                            "({}/{}) Offer applications process succeed. Offer ID: {}.",
                            counter,
                            amount,
                            offer.id
                        );
                        break value;
                    },
                    Err(_) => {
                        let error: ErrorResponse = serde_json::from_str(&text)
                            .map_err(ApiError::JsonParseFailed)?;
                        error.handle_request_limit().await;
                    },
                };
            };

            let length = dto_map.requests.len() as i32;

            for dto in dto_map.requests {
                let value = Application::try_from(dto)?;
                applications.push(value);
            }

            if length == 100 {
                parameters.last += 100;
            } else {
                break;
            }
        }
    }

    Ok(applications)
}

pub struct ApplicantsApi {
    pub id: u32,
    pub last: i32,
}

impl ApiFetcherForm for ApplicantsApi {
    fn create_form(&self) -> HashMap<String, String> {
        const ID: &str = "id";
        const LAST: &str = "last";

        let mut map = HashMap::new();

        map.insert(ID.to_string(), self.id.to_string());
        map.insert(LAST.to_string(), self.last.to_string());

        map
    }
}
