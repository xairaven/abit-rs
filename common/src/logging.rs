use chrono::{Datelike, Local, Timelike};
use log::{LevelFilter, Record};
use std::fmt::Arguments;
use thiserror::Error;

pub const DEFAULT_FORMAT: &str = "[%Y-%m-%D %H:%M %LEVEL] %MESSAGE";

#[derive(Error, Debug)]
pub enum LogError {
    #[error("IO Error. {0}")]
    IOError(#[from] std::io::Error),

    #[error("Logger initialization error. {0}")]
    SetLoggerError(log::SetLoggerError),
}

#[derive(Debug)]
pub enum LogOutput {
    File,
    Console,
}

pub struct LogSettings {
    pub app_name: String,
    pub log_level: LevelFilter,
    pub format: String,
    pub output: LogOutput,
}

impl LogSettings {
    pub fn setup(&self) -> Result<(), LogError> {
        if self.log_level.eq(&LevelFilter::Off) {
            return Ok(());
        }

        let mut dispatcher = fern::Dispatch::new().level(self.log_level).format({
            let format = self.format.to_string();
            move |out, message, record| {
                let formatted = Self::parse_format(&format, message, record);

                out.finish(format_args!("{formatted}"))
            }
        });

        match self.output {
            LogOutput::File => {
                let file_name = self.generate_file_name();
                let file = fern::log_file(file_name).map_err(LogError::IOError)?;
                dispatcher = dispatcher.chain(file);
            },
            LogOutput::Console => {
                dispatcher = dispatcher.chain(std::io::stdout());
            },
        }

        dispatcher.apply().map_err(LogError::SetLoggerError)
    }

    fn generate_file_name(&self) -> String {
        let now = Local::now();
        let date = format!(
            "{year:04}-{month:02}-{day:02}",
            year = now.year(),
            month = now.month(),
            day = now.day(),
        );

        let title = self.app_name.trim().replace(" ", "-");
        format!("{title}_{date}.log")
    }

    fn parse_format(format: &str, message: &Arguments, record: &Record) -> String {
        let mut log = format.trim().to_string();

        // Time
        let time = Local::now();
        log = log.replacen("%Y", &format!("{:0>2}", time.year()), 1);
        log = log.replacen("%m", &format!("{:0>2}", time.month()), 1);
        log = log.replacen("%D", &format!("{:0>2}", time.day()), 1);
        log = log.replacen("%H", &format!("{:0>2}", time.hour()), 1);
        log = log.replacen("%M", &format!("{:0>2}", time.minute()), 1);
        log = log.replacen("%S", &format!("{:0>2}", time.second()), 1);

        // Level
        log = log.replacen("%LEVEL", record.level().as_str(), 1);

        // Target
        log = log.replacen("%TARGET", record.target(), 1);

        // Message
        log = log.replacen("%MESSAGE", &message.to_string(), 1);

        log
    }
}
