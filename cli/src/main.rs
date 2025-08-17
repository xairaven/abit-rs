use common::logging;

#[tokio::main]
async fn main() -> () {
    let config = match common::env::from_env() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Environment file error. {}", error);
            std::process::exit(1);
        },
    };

    logging::LogSettings {
        app_name: config.app_name,
        log_level: config.log_level,
        format: config.log_format,
    }
    .setup()
    .unwrap_or_else(|error| {
        eprintln!("Logger initialization error. {}", error);
        std::process::exit(1);
    });

    log::info!("App started.");
    log::info!("Logger initialized.");

    match edbo_core::process().await {
        Ok(client) => client,
        Err(error) => {
            dbg!(error);
            return;
        },
    };
}
