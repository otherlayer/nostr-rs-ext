use nostr_rs_ext::server;
use nostr_rs_ext::config;
use clap::Parser;


#[derive(Parser)]
#[command(about = "A nostr extensions for nostr-rs-relay", author = env!("CARGO_PKG_AUTHORS"), version = env!("CARGO_PKG_VERSION"))]
pub struct CLIArgs {
    #[arg(
        short,
        long,
        help = "Use the <file name> as the location of the config file",
        required = false,
    )]
    pub config: Option<String>,
}


#[tokio::main]
async fn main() {
    let args = CLIArgs::parse();

    let config_file_arg = args.config;

    let settings = config::Settings::new(&config_file_arg);

    tracing_subscriber::fmt::try_init().unwrap();

    server::start(settings).await;
}
