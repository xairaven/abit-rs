use common::logging;
use edbo_core::EdboClient;

#[tokio::main]
async fn main() -> () {
    let config = match common::env::from_env() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("ERROR: {}", error);
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
        eprintln!("ERROR: Logger initialization failed. {}", error);
        std::process::exit(1);
    });

    let client = match EdboClient::init().await {
        Ok(client) => client,
        Err(error) => {
            dbg!(error);
            return;
        },
    };
}
