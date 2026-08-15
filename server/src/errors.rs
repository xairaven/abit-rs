use crate::config::ConfigError;
use crate::database::DbError;
use crate::logs::LogsError;
use crate::settings::RuntimeSettingsError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Configuration. {0}")]
    Config(#[from] ConfigError),

    #[error("Database. {0}")]
    Database(#[from] DbError),

    #[error("Logger. {0}")]
    Logs(#[from] LogsError),

    #[error("Settings. {0}")]
    RuntimeSettings(#[from] RuntimeSettingsError),
}
