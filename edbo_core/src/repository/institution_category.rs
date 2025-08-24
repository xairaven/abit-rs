use crate::database::Database;
use crate::model::institution_category::InstitutionCategory;
use crate::repository::{Repository, RepositoryError, RepositoryResult};
use strum::IntoEnumIterator;

pub struct InstitutionCategoryRepository<'a> {
    db: &'a Database,
}

#[async_trait::async_trait]
impl<'a> Repository<'a> for InstitutionCategoryRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }

    async fn is_empty(&self) -> RepositoryResult<bool> {
        let result = sqlx::query!("SELECT EXISTS (SELECT 1 FROM institution_category);")
            .fetch_one(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;

        Ok(!result.exists.unwrap_or(false))
    }
}

impl<'a> InstitutionCategoryRepository<'a> {
    pub async fn create(&self) -> RepositoryResult<()> {
        for category in InstitutionCategory::iter() {
            sqlx::query!(
                r#"
                INSERT INTO institution_category (id, description, code)
                VALUES ($1, $2, $3)
            "#,
                category as i8,
                category.to_string(),
                category.code()
            )
            .execute(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;
        }

        Ok(())
    }
}
