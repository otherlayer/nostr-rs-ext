use nostr_rs_ext::server;
use nostr_rs_ext::config;


#[tokio::main]
async fn main() {
    let settings = config::Settings::new(&Some("config.toml".to_string()));

    tracing_subscriber::fmt::try_init().unwrap();

    server::start(settings).await;
}
