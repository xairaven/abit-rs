use crate::database::Database;
use crate::model::region::Region;
use crate::repository::{Repository, RepositoryError, RepositoryResult};
use strum::IntoEnumIterator;

pub struct RegionRepository<'a> {
    db: &'a Database,
}

impl<'a> Repository<'a> for RegionRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }
}

impl<'a> RegionRepository<'a> {
    pub async fn create(&self) -> RepositoryResult<()> {
        for region in Region::iter() {
            sqlx::query!(
                r#"
                INSERT INTO region (id, name)
                VALUES ($1, $2)
            "#,
                region as i8,
                region.to_string()
            )
            .execute(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;
        }

        Ok(())
    }
}
