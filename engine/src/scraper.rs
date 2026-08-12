use crate::EngineConfig;
use crate::database::Database;
use crate::errors::EngineError;

#[derive(Debug)]
pub struct Scraper {
    config: EngineConfig,
}

impl Scraper {
    pub fn new(config: &EngineConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn process(&self) -> Result<(), EngineError> {
        let db = Database::init(&self.config).await?;

        // services::enum_service::EnumService::new(&db)
        //     .build()
        //     .await?;
        //
        // let institutions = services::institutions::InstitutionService::new(&db)
        //     .get()
        //     .await?;
        // let mut offers_with_institutions =
        //     services::offer_university::OfferUniversityService::new(&db)
        //         .get()
        //         .await?;
        // let offers = services::offer::OfferService::new(&db)
        //     .get(&mut offers_with_institutions)
        //     .await?;
        // let (applications, applicants) = services::applications::ApplicationService::new(&db)
        //     .get(&offers)
        //     .await?;
        //
        // let context = Context {
        //     applicants,
        //     applications,
        //     institutions,
        //     offers,
        //     offers_with_institutions,
        // };

        Ok(())
    }
}
