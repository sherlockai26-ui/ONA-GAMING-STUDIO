use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use serde_json::json;
use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex, OnceLock,
    },
};
use tokio::sync::{broadcast, mpsc};

use crate::session::manager::{authenticate_controller, mark_controller_disconnected};

static CONNECTION_EVENTS: OnceLock<broadcast::Sender<u8>> = OnceLock::new();
static INPUT_EVENTS: OnceLock<broadcast::Sender<String>> = OnceLock::new();
static ACTIVE_TRANSPORTS: OnceLock<Mutex<HashMap<String, ActiveTransport>>> = OnceLock::new();
static NEXT_CONNECTION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
struct ActiveTransport {
    connection_id: u64,
    supersede: mpsc::UnboundedSender<()>,
}

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

fn active_transports() -> &'static Mutex<HashMap<String, ActiveTransport>> {
    ACTIVE_TRANSPORTS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub async fn handler(ws: WebSocketUpgrade) -> Response {
    println!("[WEBSOCKET] Upgrade request received.");

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("[WEBSOCKET] Controller transport connected.");

    let connection_id = NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed);
    let (supersede_sender, mut supersede_receiver) = mpsc::unbounded_channel::<()>();
    let mut authenticated_session_id: Option<String> = None;
    let mut authenticated_device_id: Option<String> = None;
    let mut authenticated_player_id: Option<u8> = None;
    let mut active_transport_key: Option<String> = None;
    let mut down_buttons: HashSet<String> = HashSet::new();
    let mut joystick_active = false;
    let mut superseded = false;

    loop {
        let result = tokio::select! {
            _ = supersede_receiver.recv() => {
                superseded = true;
                println!("[ONA Controller] old transport superseded connection={connection_id}");
                break;
            }
            result = socket.recv() => result,
        };

        let Some(result) = result else {
            break;
        };

        match result {
            Ok(Message::Text(text)) => {
                let text = text.to_string();
                let Ok(mut message) = serde_json::from_str::<serde_json::Value>(&text) else {
                    continue;
                };
                let message_type = message["type"].as_str().unwrap_or_default();
                if message_type == "controller_connected" {
                    println!("[WEBSOCKET] Message received: controller_connected");
                } else {
                    println!("[WEBSOCKET] Message received: {message_type}");
                }

                if message_type == "controller_connected" {
                    let session_id = message["sessionId"].as_str().unwrap_or_default();
                    let token = message["token"].as_str().unwrap_or_default();
                    let device_id = message["deviceId"]
                        .as_str()
                        .filter(|id| !id.trim().is_empty());

                    match authenticate_controller(session_id, token, device_id) {
                        Ok(controller) => {
                            authenticated_session_id = Some(controller.session_id.clone());
                            authenticated_device_id = controller.device_id.clone();
                            authenticated_player_id = Some(controller.player_id);
                            let key = transport_key(
                                &controller.session_id,
                                controller.device_id.as_deref(),
                            );
                            if let Ok(mut transports) = active_transports().lock() {
                                if let Some(previous) = transports.remove(&key) {
                                    println!("[ONA Controller] duplicate device connection");
                                    let _ = previous.supersede.send(());
                                }
                                transports.insert(
                                    key.clone(),
                                    ActiveTransport {
                                        connection_id,
                                        supersede: supersede_sender.clone(),
                                    },
                                );
                                active_transport_key = Some(key);
                            }
                            println!(
                                "[ONA Controller] active transport connection={connection_id}"
                            );
                            let _ = connection_events().send(controller.player_id);
                            let response = json!({
                                "type": "controller_authenticated",
                                "sessionId": controller.session_id,
                                "deviceId": controller.device_id,
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
                    if !is_authoritative_transport(active_transport_key.as_deref(), connection_id) {
                        println!("[ONA Controller] stale input ignored connection={connection_id}");
                        continue;
                    }
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

    let authoritative = is_authoritative_transport(active_transport_key.as_deref(), connection_id);

    if authoritative {
        if let Some(key) = active_transport_key.as_deref() {
            if let Ok(mut transports) = active_transports().lock() {
                if transports
                    .get(key)
                    .map(|transport| transport.connection_id == connection_id)
                    .unwrap_or(false)
                {
                    transports.remove(key);
                }
            }
        }
    }

    if authoritative {
        if let Some(player_id) = authenticated_player_id {
            neutralize_input_state(player_id, joystick_active, &down_buttons);
        }
    }

    if authoritative && !superseded {
        if let Some(session_id) = authenticated_session_id {
            let _ = mark_controller_disconnected(&session_id, authenticated_device_id.as_deref());
        }
    }

    println!("[WEBSOCKET] Connection closed.");
}

fn transport_key(session_id: &str, device_id: Option<&str>) -> String {
    match device_id.filter(|id| !id.trim().is_empty()) {
        Some(device_id) => format!("{session_id}::{device_id}"),
        None => session_id.to_string(),
    }
}

fn is_authoritative_transport(key: Option<&str>, connection_id: u64) -> bool {
    let Some(key) = key else {
        return false;
    };

    active_transports()
        .lock()
        .ok()
        .and_then(|transports| transports.get(key).map(|transport| transport.connection_id))
        == Some(connection_id)
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
    use super::{
        active_transports, is_authoritative_transport, neutralize_input_state, track_input_state,
        transport_key, ActiveTransport,
    };
    use serde_json::json;
    use std::collections::HashSet;
    use tokio::sync::mpsc;

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

    #[test]
    fn newest_duplicate_device_transport_is_authoritative() {
        let key = transport_key("session-1", Some("device-1"));
        let (old_sender, mut old_receiver) = mpsc::unbounded_channel();
        let (new_sender, _new_receiver) = mpsc::unbounded_channel();

        {
            let mut transports = active_transports().lock().unwrap();
            transports.clear();
            transports.insert(
                key.clone(),
                ActiveTransport {
                    connection_id: 1,
                    supersede: old_sender,
                },
            );

            if let Some(previous) = transports.remove(&key) {
                let _ = previous.supersede.send(());
            }

            transports.insert(
                key.clone(),
                ActiveTransport {
                    connection_id: 2,
                    supersede: new_sender,
                },
            );
        }

        assert!(old_receiver.try_recv().is_ok());
        assert!(!is_authoritative_transport(Some(&key), 1));
        assert!(is_authoritative_transport(Some(&key), 2));

        active_transports().lock().unwrap().clear();
    }
}
