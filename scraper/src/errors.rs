use crate::database::DbError;
use crate::institution::errors::InstitutionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScraperError {
    #[error("Database. {0}")]
    Database(#[from] DbError),

    #[error("Institution. {0}")]
    Institution(#[from] InstitutionError),
}
