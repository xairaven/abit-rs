use crate::database::Database;
use crate::model::offers_university::OffersUniversity;
use crate::repository::{Repository, RepositoryError, RepositoryResult};
use sqlx::Row;
use std::collections::HashMap;

pub struct OfferUniversityRepository<'a> {
    db: &'a Database,
}

#[async_trait::async_trait]
impl<'a> Repository<'a> for OfferUniversityRepository<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self { db: database }
    }

    async fn is_empty(&self) -> RepositoryResult<bool> {
        let result = sqlx::query!("SELECT EXISTS (SELECT 1 FROM offers_institutions);")
            .fetch_one(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;

        Ok(!result.exists.unwrap_or(false))
    }
}

impl<'a> OfferUniversityRepository<'a> {
    pub async fn create(
        &self, offer_university_relation: &OffersUniversity,
    ) -> RepositoryResult<()> {
        for offer in &offer_university_relation.offers {
            sqlx::query!(
                r#"
                INSERT INTO offers_institutions (university_id, offer_id)
                VALUES ($1, $2)
            "#,
                offer_university_relation.university_id,
                offer
            )
            .execute(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;
        }

        Ok(())
    }

    pub async fn find_by_university_id(
        &self, id: i32,
    ) -> RepositoryResult<OffersUniversity> {
        let rows = sqlx::query!(
            r#"
            SELECT university_id, offer_id
            FROM offers_institutions
            WHERE university_id = $1
        "#,
            id
        )
        .fetch_all(&self.db.pool)
        .await
        .map_err(RepositoryError::Sql)?;

        let mut offers = Vec::new();
        for row in rows {
            let offer_id = row.offer_id;
            offers.push(offer_id);
        }

        let relation = OffersUniversity {
            university_id: id,
            offers,
        };

        Ok(relation)
    }

    pub async fn find_all(
        &self, limit: Option<i32>, offset: Option<i32>,
    ) -> RepositoryResult<Vec<OffersUniversity>> {
        let select = "SELECT university_id, offer_id FROM offers_institutions";

        let query = match (limit, offset) {
            (Some(l), Some(o)) => format!("{} LIMIT {} OFFSET {}", select, l, o),
            (Some(l), None) => format!("{} LIMIT {}", select, l),
            (None, Some(o)) => format!("{} OFFSET {}", select, o),
            (None, None) => select.to_string(),
        };

        let rows = sqlx::query(&query)
            .fetch_all(&self.db.pool)
            .await
            .map_err(RepositoryError::Sql)?;

        let mut relations: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows {
            let university_id: i32 = row.get(0);
            let offer_id: i32 = row.get(1);
            relations.entry(university_id).or_default().push(offer_id);
        }

        let mut offers_university_vec = Vec::new();
        for relation in relations {
            let offers_university = OffersUniversity {
                university_id: relation.0,
                offers: relation.1,
            };
            offers_university_vec.push(offers_university);
        }

        Ok(offers_university_vec)
    }
}
