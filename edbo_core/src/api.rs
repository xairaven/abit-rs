use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use thiserror::Error;
use url::Url;

pub mod links {
    pub const MAIN: &str = "https://vstup.edbo.gov.ua";
    pub const REGISTRY: &str = "https://registry.edbo.gov.ua/api";
}

pub const INTERVAL_FOR_REQUESTS: tokio::time::Duration =
    tokio::time::Duration::from_millis(1115);
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36 OPR/120.0.0.0";

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Failed to parse URL. {0}")]
    FailedToParseUrl(url::ParseError),

    #[error("Failed to build client. {0}")]
    FailedBuildClient(reqwest::Error),

    #[error("Error occurred while sent request. {0}")]
    RequestFailed(reqwest::Error),

    #[error("Failed to parse JSON. {0}")]
    JsonParseFailed(serde_json::Error),

    #[error("Failed to get response text.")]
    FailedToGetResponseText(reqwest::Error),

    #[error("Invalid header value. {0}")]
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),

    #[error("Failed to parse data: {0}")]
    FailedParsing(String),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
}

#[derive(Debug)]
pub enum ExportFormat {
    Excel,
    Xml,
    Json,
}

impl ExportFormat {
    pub const fn into_static_str(self) -> &'static str {
        match self {
            Self::Excel => "xlsx",
            Self::Xml => "xml",
            Self::Json => "json",
        }
    }
}

pub trait ApiFetcherUrl {
    fn append_parameters_to_url(&self, url: &mut Url);

    fn append_optional_parameter<T: Display>(
        url: &mut Url, key: &str, value: &Option<T>,
    ) {
        if let Some(value) = value {
            url.query_pairs_mut().append_pair(key, &value.to_string());
        }
    }
}

pub trait ApiFetcherForm {
    fn create_form(&self) -> HashMap<String, String>;

    fn append_option_to_form<T: Display>(
        map: &mut HashMap<String, String>, key: &str, value: &Option<T>,
    ) {
        if let Some(value) = value {
            map.insert(key.to_string(), value.to_string());
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl ErrorResponse {
    pub async fn handle_request_limit(&self) {
        log::error!("Error: {}", self.error);
        log::error!("Error Message: {}", self.message);

        if self
            .error
            .eq("Перевищено ліміт запитів. Спробуйте пізніше!")
        {
            log::warn!("Sleeping 60 secs...");
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}

pub mod applications;
pub mod institution;
pub mod offers;
pub mod offers_university;
