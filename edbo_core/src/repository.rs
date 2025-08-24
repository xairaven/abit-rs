use crate::database::Database;
use crate::error::CoreError;
use thiserror::Error;

pub type RepositoryResult<T> = Result<T, CoreError>;

#[async_trait::async_trait]
pub trait Repository<'a>: Send + Sync {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized;

    async fn is_empty(&self) -> RepositoryResult<bool>;
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
pub mod degree;
pub mod institution;
pub mod institution_category;
pub mod knowledge_field;
pub mod offer;
pub mod offers_university;
pub mod ownership_form;
pub mod priority;
pub mod region;
pub mod speciality;
pub mod status;
pub mod study_form;
