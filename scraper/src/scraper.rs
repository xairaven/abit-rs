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

        Ok(())
    }
}
