use crate::database::Database;
use crate::model::speciality::KnowledgeField;
use crate::repository::{Repository, RepositoryError, RepositoryResult};
use strum::IntoEnumIterator;

pub struct KnowledgeFieldRepository<'a> {
    db: &'a Database,
}

impl<'a> Repository<'a> for KnowledgeFieldRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }
}

impl<'a> KnowledgeFieldRepository<'a> {
    pub async fn create(&self) -> RepositoryResult<()> {
        for field in KnowledgeField::iter() {
            sqlx::query!(
                r#"
                INSERT INTO knowledge_field (code, name)
                VALUES ($1, $2)
            "#,
                field.code(),
                field.to_string()
            )
            .execute(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;
        }

        Ok(())
    }
}
