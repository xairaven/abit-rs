use crate::config::ConfigError;
use crate::logs::LogsError;
use crate::settings::RuntimeSettingsError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Configuration. {0}")]
    Config(#[from] ConfigError),

    #[error("Logger. {0}")]
    Logs(#[from] LogsError),

    #[error("Settings. {0}")]
    RuntimeSettings(#[from] RuntimeSettingsError),
}
