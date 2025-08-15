use crate::logging;
use log::LevelFilter;
use std::env;
use thiserror::Error;

#[derive(Debug)]
pub struct EnvConfig {
    pub database_url: String,
    pub log_format: String,
    pub log_level: LevelFilter,
}

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("File not found.")]
    FileNotFound(dotenvy::Error),

    #[error("Database URL is absent.")]
    DatabaseUrlNotFound,

    #[error("Unknown log level.")]
    UnknownLogLevel,
}

const DATABASE_URL_KEY: &str = "DATABASE_URL";
const LOG_FORMAT_KEY: &str = "LOG_FORMAT";
const LOG_LEVEL_KEY: &str = "LOG_LEVEL";

pub fn from_env() -> Result<EnvConfig, EnvError> {
    dotenvy::dotenv().map_err(EnvError::FileNotFound)?;

    // Loading database URL
    let database_url =
        env::var(DATABASE_URL_KEY).map_err(|_| EnvError::DatabaseUrlNotFound)?;

    // Loading log format
    let log_format =
        env::var(LOG_FORMAT_KEY).unwrap_or(String::from(logging::DEFAULT_FORMAT));

    // Loading log level
    let log_level = env::var(LOG_LEVEL_KEY)
        .unwrap_or("off".to_string())
        .parse::<LevelFilter>()
        .map_err(|_| EnvError::UnknownLogLevel)?;

    Ok(EnvConfig {
        database_url,
        log_format,
        log_level,
    })
}
