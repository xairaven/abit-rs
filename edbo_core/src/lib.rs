use crate::error::CoreError;

// Main Source: https://zakon.rada.gov.ua/laws/show/z0312-25#Text

pub async fn process(settings: InitSettings) -> Result<(), CoreError> {
    let institutions = api::institution::list().await?;
    let offers_with_institutions = api::offers_university::list().await?;

    log::info!("INSTITUTIONS: \n ----- \n {:?} \n -------", institutions);
    log::info!(
        "Offers <-> Institutions: \n ----- \n {:?} \n -------",
        offers_with_institutions
    );

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
pub mod model;

pub mod dto {
    pub mod institution;
    pub mod offers_university;
}
