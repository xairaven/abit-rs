use crate::api;
use crate::database::Database;
use crate::error::CoreError;
use crate::model::institution::Institution;
use crate::repository::Repository;
use crate::repository::institution::InstitutionRepository;
use crate::services::Service;

pub struct InstitutionService<'a> {
    institution_repository: InstitutionRepository<'a>,
}

impl<'a> Service<'a> for InstitutionService<'a> {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized,
    {
        Self {
            institution_repository: InstitutionRepository::new(database),
        }
    }
}

impl<'a> InstitutionService<'a> {
    pub async fn get(&self) -> Result<Vec<Institution>, CoreError> {
        let list = if self.institution_repository.is_empty().await? {
            log::info!("Institutions table is clear. Requesting data from API...");
            let list = api::institution::list().await?;
            for institution in list.iter() {
                self.institution_repository.create(institution).await?;
            }
            log::info!("Institutions table populated with {} records", list.len());
            list
        } else {
            log::info!("Institutions are already populated. Trying to fetch from DB...");
            let list = self.institution_repository.find_all().await?;
            log::info!(
                "Successfully fetched {} institution records from DB",
                list.len()
            );
            list
        };

        Ok(list)
    }
}
