use super::{bridge::GameInputBridge, events::OnaInputEvent};

pub fn dispatch(event: OnaInputEvent, bridge: Option<&GameInputBridge>) {
    println!("[ONA Input] {event:?}");

    if let Some(bridge) = bridge {
        bridge.send(event);
    }
}
