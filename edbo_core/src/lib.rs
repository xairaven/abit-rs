use crate::error::CoreError;

// Main Source: https://zakon.rada.gov.ua/laws/show/z0312-25#Text

pub async fn process(settings: InitSettings) -> Result<(), CoreError> {
    // let institution_list = api::institution::list().await?;

    database::init(&settings).await?;

    Ok(())
}

#[derive(Debug)]
pub struct InitSettings {
    pub database_url: String,
}

pub mod api;
pub mod database;
pub mod error;

pub mod dto {
    pub mod institution;
}
pub mod model {
    pub mod degree;
    pub mod institution;
    pub mod region;
    pub mod speciality;
}
