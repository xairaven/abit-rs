use crate::settings::RuntimeSettings;
use chrono::{Datelike, Local, Timelike};
use log::LevelFilter;
use o2o::o2o;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum_macros::{Display, EnumIter};
use thiserror::Error;

pub const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Info;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    EnumIter,
    Display,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    o2o,
)]
#[o2o(map_owned(log::LevelFilter))]
pub enum LogLevel {
    #[default]
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum LogDestination {
    #[default]
    Stdout,
    Directory(PathBuf),
}

pub struct Logger {
    log_level: LevelFilter,
    log_destination: LogDestination,
}

impl Logger {
    pub fn from_settings(settings: &RuntimeSettings) -> Self {
        Self {
            log_level: settings.log_level,
            log_destination: settings.log_destination.clone(),
        }
    }

    pub fn setup(self) -> Result<(), LogsError> {
        if self.log_level.eq(&LevelFilter::Off) {
            return Ok(());
        }

        let dispatcher = fern::Dispatch::new().level(self.log_level).format(
            move |out, message, record| {
                let time = Local::now();
                out.finish(format_args!(
                    "[{:0>2}-{:0>2}-{:0>2} {:0>2}:{:0>2} {}] {}",
                    time.year(),
                    time.month(),
                    time.day(),
                    time.hour(),
                    time.minute(),
                    record.level(),
                    message
                ))
            },
        );

        let dispatcher = match self.log_destination {
            LogDestination::Stdout => dispatcher.chain(std::io::stdout()),
            LogDestination::Directory(directory) => {
                let file_path = Self::path(directory)?;
                let file = fern::log_file(file_path).map_err(LogsError::IO)?;
                dispatcher.chain(file)
            },
        };

        dispatcher.apply().map_err(LogsError::SetLoggerError)
    }

    fn path(directory: PathBuf) -> Result<PathBuf, LogsError> {
        let name = Self::generate_file_name();
        let file_path = directory.join(name);
        Ok(file_path)
    }

    fn generate_file_name() -> String {
        let now = Local::now();
        let date = format!(
            "{year:04}-{day:02}-{month:02}",
            year = now.year(),
            day = now.day(),
            month = now.month(),
        );

        format!("{date}.log")
    }
}

#[derive(Debug, Error)]
pub enum LogsError {
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Set Logger: {0}")]
    SetLoggerError(#[from] log::SetLoggerError),
}
