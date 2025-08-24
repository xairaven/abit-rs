use crate::database::Database;
use crate::model::study_form::StudyForm;
use crate::repository::{Repository, RepositoryError, RepositoryResult};
use strum::IntoEnumIterator;

pub struct StudyFormRepository<'a> {
    db: &'a Database,
}

#[async_trait::async_trait]
impl<'a> Repository<'a> for StudyFormRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }

    async fn is_empty(&self) -> RepositoryResult<bool> {
        let result = sqlx::query!("SELECT EXISTS (SELECT 1 FROM study_form);")
            .fetch_one(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;

        Ok(!result.exists.unwrap_or(false))
    }
}

impl<'a> StudyFormRepository<'a> {
    pub async fn create(&self) -> RepositoryResult<()> {
        for form in StudyForm::iter() {
            sqlx::query!(
                r#"
                INSERT INTO study_form (id, description)
                VALUES ($1, $2)
            "#,
                form as i8,
                form.to_string()
            )
            .execute(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;
        }

        Ok(())
    }
}
