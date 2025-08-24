use crate::api::ApiError;

pub struct Client;

impl Client {
    pub fn build() -> Result<reqwest::Client, ApiError> {
        let client = reqwest::Client::builder()
            .pool_max_idle_per_host(3)
            .build()
            .map_err(ApiError::FailedBuildClient)?;

        Ok(client)
    }
}
