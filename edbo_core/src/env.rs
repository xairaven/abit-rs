use std::env;
use thiserror::Error;

#[derive(Debug)]
pub struct EnvConfig {
    database_url: String,
}

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("File not found.")]
    FileNotFound(dotenvy::Error),

    #[error("Database URL is absent.")]
    DatabaseUrlNotFound,
}

pub fn from_env() -> Result<EnvConfig, EnvError> {
    dotenvy::dotenv().map_err(EnvError::FileNotFound)?;

    // Loading database url
    let database_url =
        env::var("DATABASE_URL").map_err(|_| EnvError::DatabaseUrlNotFound)?;

    Ok(EnvConfig { database_url })
}
