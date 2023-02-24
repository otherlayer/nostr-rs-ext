use crate::grpc;
use crate::config::Settings;

pub async fn start(
    settings: Settings,
) {
    grpc::start_server(settings).await;
}
