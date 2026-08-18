use crate::database::DbError;
use crate::dto::DtoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScraperError {
    #[error("Database Error. {0}")]
    Database(#[from] DbError),

    #[error("DTO Conversion. {0}")]
    Dto(#[from] DtoError),
}
