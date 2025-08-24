use crate::database::Database;
use crate::model::ownership_form::OwnershipForm;
use crate::repository::{Repository, RepositoryError, RepositoryResult};
use strum::IntoEnumIterator;

pub struct OwnershipFormRepository<'a> {
    db: &'a Database,
}

impl<'a> Repository<'a> for OwnershipFormRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }
}

impl<'a> OwnershipFormRepository<'a> {
    pub async fn create(&self) -> RepositoryResult<()> {
        for form in OwnershipForm::iter() {
            sqlx::query!(
                r#"
                INSERT INTO ownership_form (id, description)
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
