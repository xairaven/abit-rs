use crate::database::Database;
use crate::model::status::ApplicationStatus;
use crate::repository::{Repository, RepositoryError, RepositoryResult};
use strum::IntoEnumIterator;

pub struct ApplicationStatusRepository<'a> {
    db: &'a Database,
}

impl<'a> Repository<'a> for ApplicationStatusRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }
}

impl<'a> ApplicationStatusRepository<'a> {
    pub async fn create(&self) -> RepositoryResult<()> {
        for status in ApplicationStatus::iter() {
            sqlx::query!(
                r#"
                INSERT INTO application_status (id, description)
                VALUES ($1, $2)
            "#,
                status as i8,
                status.to_string()
            )
            .execute(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;
        }

        Ok(())
    }
}
