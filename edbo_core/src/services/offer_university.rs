use crate::api;
use crate::database::Database;
use crate::error::CoreError;
use crate::model::offers_university::OffersUniversity;
use crate::repository::Repository;
use crate::repository::offers_university::OfferUniversityRepository;
use crate::services::Service;

pub struct OfferUniversityService<'a> {
    repo: OfferUniversityRepository<'a>,
}

impl<'a> Service<'a> for OfferUniversityService<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self {
            repo: OfferUniversityRepository::new(database),
        }
    }
}

impl<'a> OfferUniversityService<'a> {
    pub async fn get(&self) -> Result<Vec<OffersUniversity>, CoreError> {
        let list = if self.repo.is_empty().await? {
            log::info!(
                "Offers <-> Institutions table is clear. Requesting data from API..."
            );
            let list = api::offers_university::list().await?;
            for relation in list.iter() {
                self.repo.create(relation).await?;
            }
            log::info!(
                "Offers <-> Institutions table populated with {} records",
                list.len()
            );
            list
        } else {
            log::info!(
                "Offers <-> Institutions are already populated. Trying to fetch from DB..."
            );
            let list = self.repo.find_all().await?;
            log::info!(
                "Successfully fetched {} Offers <-> Institutions relation records from DB",
                list.len()
            );
            list
        };

        Ok(list)
    }
}
