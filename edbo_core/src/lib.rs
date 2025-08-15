use crate::error::CoreError;

pub struct EdboClient {}

impl EdboClient {
    pub async fn init() -> Result<Self, CoreError> {
        let config = env::from_env()?;
        dbg!(config);

        Ok(EdboClient {})
    }
}

mod env;
mod error;
