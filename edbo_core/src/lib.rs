use crate::error::CoreError;

pub struct EdboClient {}

impl EdboClient {
    pub async fn init() -> Result<Self, CoreError> {
        Ok(EdboClient {})
    }
}

mod error;
