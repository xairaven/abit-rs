use crate::config::Config;
use crate::logs;
use crate::logs::LogDestination;
use log::LevelFilter;
use thiserror::Error;

#[derive(Debug)]
pub struct RuntimeSettings {
    pub database_url: String,
    pub log_level: LevelFilter,
    pub log_destination: LogDestination,
}

impl TryFrom<Config> for RuntimeSettings {
    type Error = RuntimeSettingsError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        if value.database_url.is_empty() {
            return Err(Self::Error::DatabaseUrlEmpty);
        }
        let database_url = value.database_url;

        let log_level: LevelFilter =
            value.log_level.map_or(logs::DEFAULT_LOG_LEVEL, Into::into);

        let log_destination = match value.log_directory {
            Some(directory) => {
                if !directory.is_dir() {
                    return Err(Self::Error::LogPathNotDirectory);
                }
                LogDestination::Directory(directory)
            },
            None => LogDestination::Stdout,
        };

        Ok(Self {
            database_url,
            log_level,
            log_destination,
        })
    }
}

#[derive(Debug, Error)]
pub enum RuntimeSettingsError {
    #[error("Database URL field is empty.")]
    DatabaseUrlEmpty,

    #[error("Provided log destination is not a directory.")]
    LogPathNotDirectory,
}
