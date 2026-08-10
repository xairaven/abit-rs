use crate::logs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<logs::LogLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_directory: Option<PathBuf>,
}

impl Config {
    const FILENAME: &str = "config.toml";

    fn path() -> Result<PathBuf, ConfigError> {
        let mut current_dir = std::env::current_exe().map_err(ConfigError::IO)?;
        current_dir.pop(); // Remove executable name

        std::fs::create_dir_all(&current_dir).map_err(ConfigError::IO)?;

        Ok(current_dir.join(Self::FILENAME))
    }

    pub fn from_file() -> Result<Self, ConfigError> {
        let path = Self::path()?;
        let text = std::fs::read_to_string(&path);
        let text = match text {
            Ok(text) => text,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let config = Self::default();
                config.save_to_file()?;
                let error = ConfigError::FileNotFound(path);
                return Err(error);
            },
            Err(error) => {
                let error = ConfigError::from(error);
                return Err(error);
            },
        };
        let config = toml::from_str(&text).map_err(ConfigError::Deserialization)?;
        Ok(config)
    }

    pub fn save_to_file(&self) -> Result<(), ConfigError> {
        let data = toml::to_string(&self).map_err(ConfigError::Serialization)?;
        let path = Self::path()?;

        std::fs::write(path, data).map_err(ConfigError::IO)?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(
        "File is not found by path \"{0}\". Utility created it for you there. Please, read documentation and fill desired configuration."
    )]
    FileNotFound(PathBuf),

    #[error("Failed to serialize. {0}")]
    Serialization(#[from] toml::ser::Error),

    #[error("Failed to deserialize. {0}")]
    Deserialization(#[from] toml::de::Error),

    #[error("I/O. {0}")]
    IO(#[from] std::io::Error),
}
