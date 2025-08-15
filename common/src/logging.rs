use chrono::{Datelike, Local, Timelike};
use log::{LevelFilter, Record};
use std::fmt::Arguments;
use thiserror::Error;

pub const DEFAULT_FORMAT: &str = "[$Y-$m-$D $H:$M $LEVEL] $MESSAGE";

#[derive(Error, Debug)]
pub enum LogError {
    #[error("IO Error. {0}")]
    IOError(#[from] std::io::Error),

    #[error("Logger initialization error. {0}")]
    SetLoggerError(log::SetLoggerError),
}

pub fn setup(log_level: &LevelFilter, format: &str) -> Result<(), LogError> {
    if log_level.eq(&LevelFilter::Off) {
        return Ok(());
    }

    let file_name = generate_file_name("CLI-EDBO");
    let file = fern::log_file(file_name).map_err(LogError::IOError)?;

    fern::Dispatch::new()
        .level(*log_level)
        .format({
            let format = format.to_string();
            move |out, message, record| {
                let formatted = parse_format(format.clone(), message, record);

                out.finish(format_args!("{formatted}"))
            }
        })
        .chain(file)
        .apply()
        .map_err(LogError::SetLoggerError)
}

pub fn generate_file_name(title: &str) -> String {
    let now = Local::now();
    let date = format!(
        "{year:04}-{month:02}-{day:02}",
        year = now.year(),
        month = now.month(),
        day = now.day(),
    );

    let title_formatted = title.trim().replace(" ", "-");
    format!("{title_formatted}_{date}.log")
}

pub fn parse_format(format: String, message: &Arguments, record: &Record) -> String {
    let mut log = format.trim().to_string();

    // Time
    let time = Local::now();
    log = log.replacen("$Y", &format!("{:0>2}", time.year()), 1);
    log = log.replacen("$m", &format!("{:0>2}", time.month()), 1);
    log = log.replacen("$D", &format!("{:0>2}", time.day()), 1);
    log = log.replacen("$H", &format!("{:0>2}", time.hour()), 1);
    log = log.replacen("$M", &format!("{:0>2}", time.minute()), 1);
    log = log.replacen("$S", &format!("{:0>2}", time.second()), 1);

    // Level
    log = log.replacen("$LEVEL", record.level().as_str(), 1);

    // Target
    log = log.replacen("$TARGET", record.target(), 1);

    // Message
    log = log.replacen("$MESSAGE", &message.to_string(), 1);

    log
}
