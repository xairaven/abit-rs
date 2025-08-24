use crate::database::Database;
use crate::model::degree::Degree;
use crate::repository::{Repository, RepositoryError, RepositoryResult};
use strum::IntoEnumIterator;

pub struct DegreeRepository<'a> {
    db: &'a Database,
}

impl<'a> Repository<'a> for DegreeRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }
}

impl<'a> DegreeRepository<'a> {
    pub async fn create(&self) -> RepositoryResult<()> {
        for degree in Degree::iter() {
            sqlx::query!(
                r#"
                INSERT INTO degree (id, description)
                VALUES ($1, $2)
            "#,
                degree as i8,
                degree.to_string()
            )
            .execute(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;
        }

        Ok(())
    }
}
