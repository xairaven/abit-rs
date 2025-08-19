use crate::api;
use crate::database::DbError;
use crate::model::ModelError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("API Error. {0}")]
    Api(#[from] api::ApiError),

    #[error("Database Error. {0}")]
    DbError(#[from] DbError),

    #[error("Model Error. {0}")]
    ModelError(#[from] ModelError),
}
