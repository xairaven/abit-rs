use crate::context::Context;
use crate::database::Database;
use crate::error::CoreError;
use crate::services::Service;

// Main Source: https://zakon.rada.gov.ua/laws/show/z0312-25#Text

pub async fn process(settings: InitSettings) -> Result<(), CoreError> {
    let db = Database::init(&settings).await?;

    services::enum_service::EnumService::new(&db)
        .build()
        .await?;

    let institutions = services::institutions::InstitutionService::new(&db)
        .get()
        .await?;
    let mut offers_with_institutions =
        services::offer_university::OfferUniversityService::new(&db)
            .get()
            .await?;
    let offers = services::offer::OfferService::new(&db)
        .get(&mut offers_with_institutions)
        .await?;
    let (applications, applicants) = services::applications::ApplicationService::new(&db)
        .get(&offers)
        .await?;

    let context = Context {
        applicants,
        applications,
        institutions,
        offers,
        offers_with_institutions,
    };

    Ok(())
}

#[derive(Debug)]
pub struct InitSettings {
    pub database_url: String,
}

pub mod api;
pub mod context;
pub mod crypto;
pub mod database;
pub mod error;
pub mod model;
pub mod repository;
pub mod request;
pub mod services;

pub mod dto {
    pub mod application;
    pub mod institution;
    pub mod offers_university;
}
