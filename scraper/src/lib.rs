// Main Source: https://zakon.rada.gov.ua/laws/show/z0312-25#Text

pub use crate::errors::ScraperError;

use crate::database::Database;
use sqlx::PgPool;

#[derive(Debug)]
pub struct Scraper {
    database: Database,
}

impl Scraper {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            database: Database::new(pool.clone()),
        }
    }

    pub async fn process(&self) -> Result<(), ScraperError> {
        Database::configure(&self.database).await?;

        Ok(())
    }
}

mod database;
mod dto;
mod errors;
