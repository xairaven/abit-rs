use crate::config::Config;
use crate::database::Database;
use crate::logs::Logger;
use crate::settings::RuntimeSettings;
use scraper::Scraper;

#[tokio::main]
async fn main() -> () {
    let config = Config::from_file().unwrap_or_else(|error| {
        eprintln!("Error occurred. {error}");
        std::process::exit(1);
    });

    let runtime_settings = RuntimeSettings::try_from(config).unwrap_or_else(|error| {
        eprintln!("Error occurred. {error}");
        std::process::exit(1);
    });

    Logger::from_settings(&runtime_settings)
        .setup()
        .unwrap_or_else(|error| {
            eprintln!("Error occurred. {error}");
            std::process::exit(1);
        });

    log::info!("Configuration successfully loaded.");
    log::info!("Runtime settings: {runtime_settings:?}");
    log::info!("Logger successfully initialized.");

    let db = Database::init(&runtime_settings)
        .await
        .unwrap_or_else(|error| {
            eprintln!("Error occurred. {error}");
            std::process::exit(1);
        });

    log::info!("Database successfully initialized.");

    log::info!("Starting process...");

    Scraper::new(&db.pool)
        .process()
        .await
        .unwrap_or_else(|error| {
            eprintln!("Error occurred. {error}");
            std::process::exit(1);
        });
}

mod config;
mod database;
mod errors;
mod logs;
mod settings;
