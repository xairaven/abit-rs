use crate::request;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Request Error. {0}")]
    RequestError(#[from] request::RequestError),
}
