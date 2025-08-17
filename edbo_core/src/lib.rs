use crate::error::CoreError;

pub async fn process() -> Result<(), CoreError> {
    request::universities::list()
        .await
        .map_err(CoreError::RequestError)
}

pub mod error;

pub mod dto {
    pub mod universities;
}
pub mod model {
    pub mod region;
    pub mod university;
}
pub mod request;
