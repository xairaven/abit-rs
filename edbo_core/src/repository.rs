use crate::database::Database;
use crate::error::CoreError;
use thiserror::Error;

pub type RepositoryResult<T> = Result<T, CoreError>;

pub trait Repository<'a>: Send + Sync {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized;
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("SQL: {0}")]
    Sql(#[from] sqlx::Error),

    #[error("Json. {0}")]
    Json(#[from] serde_json::Error),
}

pub mod applicant;
pub mod application;
pub mod institution;
