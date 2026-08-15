use crate::database::DbError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScraperError {
    #[error("Database Error. {0}")]
    Database(#[from] DbError),
}
