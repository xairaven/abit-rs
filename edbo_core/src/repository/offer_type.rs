use crate::database::Database;
use crate::model::offer_type::OfferType;
use crate::repository::{Repository, RepositoryError, RepositoryResult};
use strum::IntoEnumIterator;

pub struct OfferTypeRepository<'a> {
    db: &'a Database,
}

#[async_trait::async_trait]
impl<'a> Repository<'a> for OfferTypeRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }

    async fn is_empty(&self) -> RepositoryResult<bool> {
        let result = sqlx::query!("SELECT EXISTS (SELECT 1 FROM offer_type);")
            .fetch_one(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;

        Ok(!result.exists.unwrap_or(false))
    }
}

impl<'a> OfferTypeRepository<'a> {
    pub async fn create(&self) -> RepositoryResult<()> {
        for offer_type in OfferType::iter() {
            sqlx::query!(
                r#"
                INSERT INTO offer_type (id, description)
                VALUES ($1, $2)
            "#,
                (offer_type as i16),
                offer_type.to_string()
            )
            .execute(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;
        }

        Ok(())
    }
}
