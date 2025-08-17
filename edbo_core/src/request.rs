use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Failed to parse URL. {0}")]
    FailedToParseUrl(url::ParseError),

    #[error("Failed to build client. {0}")]
    FailedBuildClient(reqwest::Error),

    #[error("Error occurred while sent request. {0}")]
    RequestFailed(reqwest::Error),

    #[error("Failed to parse JSON. {0}")]
    JsonParseFailed(reqwest::Error),
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

pub mod institution;
