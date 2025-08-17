use crate::error::CoreError;

// Main Source: https://zakon.rada.gov.ua/laws/show/z0312-25#Text

pub async fn process() -> Result<(), CoreError> {
    let a = api::institution::list().await?;

    dbg!(a);

    Ok(())
}

pub mod api;
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
