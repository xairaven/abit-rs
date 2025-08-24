use crate::database::Database;
use crate::model::priority::Priority;
use crate::repository::{Repository, RepositoryError, RepositoryResult};
use strum::IntoEnumIterator;

pub struct PriorityRepository<'a> {
    db: &'a Database,
}

impl<'a> Repository<'a> for PriorityRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }
}

impl<'a> PriorityRepository<'a> {
    pub async fn create(&self) -> RepositoryResult<()> {
        for priority in Priority::iter() {
            sqlx::query!(
                r#"
                INSERT INTO priority (id, key)
                VALUES ($1, $2)
            "#,
                priority as i8,
                priority.to_string()
            )
            .execute(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;
        }

        Ok(())
    }
}
