use edbo_core::EdboClient;

#[tokio::main]
async fn main() -> () {
    let client = match EdboClient::init().await {
        Ok(client) => client,
        Err(error) => {
            dbg!(error);
            return;
        },
    };
}
