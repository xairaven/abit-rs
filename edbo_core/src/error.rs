use crate::api;
use crate::model::institution::InstitutionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("API Error. {0}")]
    Api(#[from] api::ApiError),

    #[error("Institution Error. {0}")]
    Institution(#[from] InstitutionError),
}
