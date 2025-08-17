use crate::error::CoreError;

pub async fn process() -> Result<(), CoreError> {
    let a = request::institution::list().await?;

    dbg!(a);

    Ok(())
}

pub mod error;

pub mod dto {
    pub mod institution;
}
pub mod model {
    pub mod degree;
    pub mod institution;
    pub mod region;
}
pub mod request;
