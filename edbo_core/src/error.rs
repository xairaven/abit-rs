use crate::model::institution::InstitutionError;
use crate::request;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Request Error. {0}")]
    Request(#[from] request::RequestError),

    #[error("Institution Error. {0}")]
    Institution(#[from] InstitutionError),
}
