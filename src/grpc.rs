
use crate::config::Settings;
use tracing::info;
use crate::plugins::{ load_plugins, ExtPlugin};

use tonic::{transport::Server, Request, Response, Status};

use nostr_rs_proto::nauthz_grpc::authorization_server::{Authorization, AuthorizationServer};
use nostr_rs_proto::nauthz_grpc::{Decision, EventReply, EventRequest};


pub struct EventAuthz {
    plugins: Vec<ExtPlugin>,
}

const DENY: i32 = Decision::Deny as i32;

#[tonic::async_trait]
impl Authorization for EventAuthz {
    async fn event_admit(
        &self,
        request: Request<EventRequest>,
    ) -> Result<Response<EventReply>, Status> {
        let req = request.into_inner();
        
        for ext_plugin in &self.plugins {
            let plugin_reply = ext_plugin.plugin.admit_event(&req);

            if plugin_reply.decision == DENY {
                return Ok(Response::new(plugin_reply));
            }
        }
        
        Ok(Response::new(EventReply {
            decision: Decision::Permit as i32,
            message: None,
        }))
    }
}

pub async fn start_server(
    settings: Settings,
) {
    let addr = format!(
        "{}:{}",
        settings.network.address.trim(),
        settings.network.port
    );

    let plugins = load_plugins(settings.plugins.folder).await;

    let socket_addr = addr.parse().expect("listening address not valid");

    info!("Listening on: {}", socket_addr);

    let auth_plugins = EventAuthz {
        plugins
    };

    Server::builder()
        .add_service(AuthorizationServer::new(auth_plugins))
        .serve(socket_addr)
        .await.ok();

    info!("Stopped")
}