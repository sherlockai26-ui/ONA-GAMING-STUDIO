use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use serde_json::json;
use std::{collections::HashSet, sync::OnceLock};
use tokio::sync::broadcast;

use crate::session::manager::{authenticate_controller, mark_controller_disconnected};

static CONNECTION_EVENTS: OnceLock<broadcast::Sender<u8>> = OnceLock::new();
static INPUT_EVENTS: OnceLock<broadcast::Sender<String>> = OnceLock::new();

fn connection_events() -> &'static broadcast::Sender<u8> {
    CONNECTION_EVENTS.get_or_init(|| broadcast::channel(16).0)
}

pub fn subscribe_connections() -> broadcast::Receiver<u8> {
    connection_events().subscribe()
}

fn input_events() -> &'static broadcast::Sender<String> {
    INPUT_EVENTS.get_or_init(|| broadcast::channel(64).0)
}

pub fn subscribe_inputs() -> broadcast::Receiver<String> {
    input_events().subscribe()
}

pub async fn handler(ws: WebSocketUpgrade) -> Response {
    println!("[WEBSOCKET] Upgrade request received.");

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("[WEBSOCKET] Controller transport connected.");

    let mut authenticated_session_id: Option<String> = None;
    let mut authenticated_player_id: Option<u8> = None;
    let mut down_buttons: HashSet<String> = HashSet::new();
    let mut joystick_active = false;

    while let Some(result) = socket.recv().await {
        match result {
            Ok(Message::Text(text)) => {
                let text = text.to_string();
                println!("[WEBSOCKET] Message received: {text}");

                let Ok(mut message) = serde_json::from_str::<serde_json::Value>(&text) else {
                    continue;
                };
                let message_type = message["type"].as_str().unwrap_or_default();

                if message_type == "controller_connected" {
                    let session_id = message["sessionId"].as_str().unwrap_or_default();
                    let token = message["token"].as_str().unwrap_or_default();

                    match authenticate_controller(session_id, token) {
                        Ok(controller) => {
                            authenticated_session_id = Some(controller.session_id.clone());
                            authenticated_player_id = Some(controller.player_id);
                            let _ = connection_events().send(controller.player_id);
                            let response = json!({
                                "type": "controller_authenticated",
                                "sessionId": controller.session_id,
                                "playerId": controller.player_id
                            });
                            let _ = socket
                                .send(Message::Text(response.to_string().into()))
                                .await;
                        }
                        Err(reason) => {
                            println!("[ONA Controller] session expired reason={reason}");
                            let response = json!({
                                "type": "controller_auth_rejected",
                                "reason": reason
                            });
                            let _ = socket
                                .send(Message::Text(response.to_string().into()))
                                .await;
                            break;
                        }
                    }

                    continue;
                }

                if message_type == "ping" {
                    let response = json!({
                        "type": "pong",
                        "timestamp": message["timestamp"].clone()
                    });
                    let _ = socket
                        .send(Message::Text(response.to_string().into()))
                        .await;
                    continue;
                }

                if message_type == "input" && authenticated_player_id.is_some() {
                    let player_id = authenticated_player_id.unwrap();
                    message["playerId"] = json!(player_id);
                    track_input_state(&message, &mut down_buttons, &mut joystick_active);
                    let _ = input_events().send(message.to_string());
                }
            }

            Ok(Message::Binary(data)) => {
                println!("[WEBSOCKET] Binary message received: {} bytes", data.len());
            }

            Ok(Message::Ping(data)) => {
                println!("[WEBSOCKET] Ping received: {} bytes", data.len());
            }

            Ok(Message::Pong(data)) => {
                println!("[WEBSOCKET] Pong received: {} bytes", data.len());
            }

            Ok(Message::Close(frame)) => {
                println!("[WEBSOCKET] Controller transport disconnected: {:?}", frame);
                break;
            }

            Err(error) => {
                println!("[WEBSOCKET] Connection error: {error}");
                break;
            }
        }
    }

    if let Some(player_id) = authenticated_player_id {
        neutralize_input_state(player_id, joystick_active, &down_buttons);
    }

    if let Some(session_id) = authenticated_session_id {
        let _ = mark_controller_disconnected(&session_id);
    }

    println!("[WEBSOCKET] Connection closed.");
}

fn track_input_state(
    message: &serde_json::Value,
    down_buttons: &mut HashSet<String>,
    joystick_active: &mut bool,
) {
    if message["control"]
        .as_str()
        .map(str::to_uppercase)
        .as_deref()
        == Some("JOYSTICK")
    {
        let x = message["x"].as_f64().unwrap_or(0.0);
        let y = message["y"].as_f64().unwrap_or(0.0);
        *joystick_active = x.abs() > 0.001 || y.abs() > 0.001;
        return;
    }

    let Some(button) = message["button"].as_str() else {
        return;
    };
    let state = message["state"].as_str().unwrap_or_default();

    if state.eq_ignore_ascii_case("down") || state.eq_ignore_ascii_case("pressed") {
        down_buttons.insert(button.to_uppercase());
    } else if state.eq_ignore_ascii_case("up") || state.eq_ignore_ascii_case("released") {
        down_buttons.remove(&button.to_uppercase());
    }
}

fn neutralize_input_state(player_id: u8, joystick_active: bool, down_buttons: &HashSet<String>) {
    if joystick_active {
        let neutral = json!({
            "type": "input",
            "playerId": player_id,
            "control": "JOYSTICK",
            "x": 0.0,
            "y": 0.0
        });
        let _ = input_events().send(neutral.to_string());
    }

    for button in down_buttons {
        let release = json!({
            "type": "input",
            "playerId": player_id,
            "button": button,
            "state": "up"
        });
        let _ = input_events().send(release.to_string());
    }

    if joystick_active || !down_buttons.is_empty() {
        println!("[ONA Controller] input state neutralized");
    }
}

#[cfg(test)]
mod tests {
    use super::{neutralize_input_state, track_input_state};
    use serde_json::json;
    use std::collections::HashSet;

    #[test]
    fn tracks_down_buttons_and_releases() {
        let mut buttons = HashSet::new();
        let mut joystick_active = false;

        track_input_state(
            &json!({"type":"input","button":"A","state":"down"}),
            &mut buttons,
            &mut joystick_active,
        );
        assert!(buttons.contains("A"));

        track_input_state(
            &json!({"type":"input","button":"A","state":"up"}),
            &mut buttons,
            &mut joystick_active,
        );
        assert!(buttons.is_empty());
    }

    #[test]
    fn tracks_non_neutral_joystick() {
        let mut buttons = HashSet::new();
        let mut joystick_active = false;

        track_input_state(
            &json!({"type":"input","control":"JOYSTICK","x":0.5,"y":0.0}),
            &mut buttons,
            &mut joystick_active,
        );
        assert!(joystick_active);

        track_input_state(
            &json!({"type":"input","control":"JOYSTICK","x":0.0,"y":0.0}),
            &mut buttons,
            &mut joystick_active,
        );
        assert!(!joystick_active);
    }

    #[test]
    fn neutralizing_empty_state_is_noop() {
        neutralize_input_state(1, false, &HashSet::new());
    }
}
