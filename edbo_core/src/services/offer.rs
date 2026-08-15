use crate::api;
use crate::database::Database;
use crate::error::CoreError;
use crate::model::offer::Offer;
use crate::model::offers_university::OffersUniversity;
use crate::repository::Repository;
use crate::repository::offer::OfferRepository;
use crate::services::Service;

pub struct OfferService<'a> {
    repo: OfferRepository<'a>,
}

impl<'a> Service<'a> for OfferService<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self {
            repo: OfferRepository::new(database),
        }
    }
}

impl<'a> OfferService<'a> {
    pub async fn get(
        &self, offers_with_institutions: &mut [OffersUniversity],
    ) -> Result<Vec<Offer>, CoreError> {
        let list = if self.repo.is_empty().await? {
            log::info!("Offers table is clear. Requesting data from API...");
            let list = api::offers::list(offers_with_institutions).await?;
            for offer in list.iter() {
                self.repo.create(offer).await?;
            }
            log::info!("Offers table populated with {} records", list.len());
            list
        } else {
            log::info!("Offers table are already populated. Trying to fetch from DB...");
            let list = self.repo.find_all().await?;
            log::info!("Successfully fetched {} offer records from DB", list.len());
            list
        };

        Ok(list)
    }
}
