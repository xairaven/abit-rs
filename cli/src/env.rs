use crate::logging;
use common::logging::LogOutput;
use log::LevelFilter;
use std::env;
use thiserror::Error;

#[derive(Debug)]
pub struct EnvConfig {
    pub app_name: String,
    pub database_url: String,
    pub log_format: String,
    pub log_level: LevelFilter,
    pub log_output: LogOutput,
}

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("{0}")]
    Inner(dotenvy::Error),

    #[error("App name value is not found")]
    AppNameNotFound,

    #[error("Database URL is absent.")]
    DatabaseUrlNotFound,

    #[error("Unknown log level.")]
    UnknownLogLevel,

    #[error("Unknown log output target.")]
    UnknownLogOutput,
}

pub fn from_env() -> Result<EnvConfig, EnvError> {
    dotenvy::dotenv().map_err(EnvError::Inner)?;

    // Loading app name
    const APP_NAME_KEY: &str = "APP_NAME";
    let app_name = env::var(APP_NAME_KEY).map_err(|_| EnvError::AppNameNotFound)?;

    // Loading database URL
    const DATABASE_URL_KEY: &str = "DATABASE_URL";
    let database_url =
        env::var(DATABASE_URL_KEY).map_err(|_| EnvError::DatabaseUrlNotFound)?;

    // Loading log format
    const LOG_FORMAT_KEY: &str = "LOG_FORMAT";
    let log_format =
        env::var(LOG_FORMAT_KEY).unwrap_or(String::from(logging::DEFAULT_FORMAT));

    // Loading log level
    const LOG_LEVEL_KEY: &str = "LOG_LEVEL";
    let log_level = env::var(LOG_LEVEL_KEY)
        .unwrap_or("off".to_string())
        .parse::<LevelFilter>()
        .map_err(|_| EnvError::UnknownLogLevel)?;

    // Loading log output target
    const LOG_OUTPUT_TARGET_KEY: &str = "LOG_OUTPUT_TARGET";
    let log_output = match env::var(LOG_OUTPUT_TARGET_KEY)
        .unwrap_or("file".to_string())
        .trim()
        .to_lowercase()
        .as_str()
    {
        "file" => LogOutput::File,
        "console" => LogOutput::Console,
        _ => return Err(EnvError::UnknownLogOutput),
    };

    Ok(EnvConfig {
        app_name,
        database_url,
        log_format,
        log_level,
        log_output,
    })
}
