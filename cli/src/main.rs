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

    // Logging setup
    logging::setup(&config.log_level, &config.log_format).unwrap_or_else(|error| {
        println!("ERROR: Logger initialization failed. {}", error);
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
