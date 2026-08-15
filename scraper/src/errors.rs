use crate::database::DbError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Database Error. {0}")]
    Db(#[from] DbError),
}
