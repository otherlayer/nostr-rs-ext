use nostr_rs_plugin::Plugin;
use nostr_rs_proto::nauthz_grpc::{EventRequest, EventReply, Decision};

struct Welcome;

impl Plugin for Welcome {
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
}

#[no_mangle]
pub fn get_plugin() -> *mut dyn Plugin {
    // Return a raw pointer to an instance of our plugin
    Box::into_raw(Box::new(Welcome {}))
}
