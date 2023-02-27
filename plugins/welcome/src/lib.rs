use nostr_rs_plugin::Plugin;
use nostr_rs_proto::nauthz_grpc::{EventRequest, EventReply, Decision};

struct Welcome;

// wip
impl Plugin for Welcome {
    fn start(&self) {}

    fn name(&self) -> String {
        return "Welcome".to_owned();
    }

    fn admit_event(&self, request: &EventRequest) -> EventReply {
        let opt_event = &request.event;

        return EventReply {
            decision: Decision::Permit as i32,
            message: Some(format!("Welcome")),
        };
    }

    fn stop(&self) {}
}

#[no_mangle]
pub fn get_plugin() -> *mut dyn Plugin {
    // Return a raw pointer to an instance of our plugin
    Box::into_raw(Box::new(Welcome {}))
}
