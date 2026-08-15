use crate::database::Database;
use crate::model::speciality::Speciality;
use crate::repository::{Repository, RepositoryError, RepositoryResult};
use strum::IntoEnumIterator;

pub struct SpecialityRepository<'a> {
    db: &'a Database,
}

#[async_trait::async_trait]
impl<'a> Repository<'a> for SpecialityRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }

    async fn is_empty(&self) -> RepositoryResult<bool> {
        let result = sqlx::query!("SELECT EXISTS (SELECT 1 FROM speciality);")
            .fetch_one(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;

        Ok(!result.exists.unwrap_or(false))
    }
}

impl<'a> SpecialityRepository<'a> {
    pub async fn create(&self) -> RepositoryResult<()> {
        for speciality in Speciality::iter() {
            sqlx::query!(
                r#"
                INSERT INTO speciality (code, name, knowledge_field)
                VALUES ($1, $2, $3)
            "#,
                speciality.code(),
                speciality.title(),
                speciality.knowledge_field().code()
            )
            .execute(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;
        }

        Ok(())
    }
}
