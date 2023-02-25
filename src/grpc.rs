use crate::config::Settings;
use tracing::info;

use tonic::{transport::Server, Request, Response, Status};

use nostr_rs_proto::nauthz_grpc::authorization_server::{Authorization, AuthorizationServer};
use nostr_rs_proto::nauthz_grpc::{Event, Decision, EventReply, EventRequest};


#[derive(Default)]
pub struct EventAuthz {
    allowed_kinds: Vec<u64>,
}

#[tonic::async_trait]
impl Authorization for EventAuthz {
    async fn event_admit(
        &self,
        request: Request<EventRequest>,
    ) -> Result<Response<EventReply>, Status> {
        let reply;
        let req = request.into_inner();
        let event: Event = req.event.unwrap();
        
        let content_prefix: String = event.content.chars().take(40).collect();
        println!("recvd event, [kind={}, origin={:?}, nip05_domain={:?}, tag_count={}, content_sample={:?}]",
                 event.kind, req.origin, req.nip05.map(|x| x.domain), event.tags.len(), content_prefix);

        if self.allowed_kinds.contains(&event.kind) {
            println!("This looks fine! (kind={})", event.kind);
            reply = EventReply {
                decision: Decision::Permit as i32,
                message: None,
            };
        } else {
            println!("Blocked! (kind={})", event.kind);
            reply = EventReply {
                decision: Decision::Deny as i32,
                message: Some(format!("kind {} not permitted", event.kind)),
            };
        }
        Ok(Response::new(reply))
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

    let socket_addr = addr.parse().expect("listening address not valid");

    info!("Listening on: {}", socket_addr);

    let checker = EventAuthz {
        allowed_kinds: vec![0, 1, 2, 3],
    };

    Server::builder()
        .add_service(AuthorizationServer::new(checker))
        .serve(socket_addr)
        .await.ok();

    info!("Stopped")
}