// websocket.rs
// Servidor WebSocket de ONA Core.
// Recibe conexiones de los controladores móviles.

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use std::sync::OnceLock;
use tokio::sync::broadcast;

static CONNECTION_EVENTS: OnceLock<broadcast::Sender<()>> = OnceLock::new();
static INPUT_EVENTS: OnceLock<broadcast::Sender<String>> = OnceLock::new();

fn connection_events() -> &'static broadcast::Sender<()> {
    CONNECTION_EVENTS.get_or_init(|| broadcast::channel(16).0)
}

pub fn subscribe_connections() -> broadcast::Receiver<()> {
    connection_events().subscribe()
}

fn input_events() -> &'static broadcast::Sender<String> {
    INPUT_EVENTS.get_or_init(|| broadcast::channel(64).0)
}

pub fn subscribe_inputs() -> broadcast::Receiver<String> {
    input_events().subscribe()
}

/// Punto de entrada del WebSocket.
///
/// El HTTP server utiliza esta función para convertir
/// una petición HTTP en una conexión WebSocket.
pub async fn handler(ws: WebSocketUpgrade) -> Response {
    println!("[WEBSOCKET] Upgrade request received.");

    ws.on_upgrade(handle_socket)
}

/// Maneja una conexión WebSocket individual.
async fn handle_socket(mut socket: WebSocket) {
    println!("[WEBSOCKET] Controller connected.");
    let _ = connection_events().send(());

    while let Some(result) = socket.recv().await {
        match result {
            Ok(Message::Text(text)) => {
                let text = text.to_string();

                println!("[WEBSOCKET] Message received: {}", text);

                if serde_json::from_str::<serde_json::Value>(&text)
                    .ok()
                    .and_then(|message| message["type"].as_str().map(str::to_owned))
                    .as_deref()
                    == Some("input")
                {
                    let _ = input_events().send(text.clone());
                }

                // Por ahora respondemos al controlador.
                // Esto nos permitirá comprobar que la conexión
                // funciona antes de conectar el sistema de inputs.
                if let Err(error) = socket
                    .send(Message::Text(format!("ONA_ACK: {}", text).into()))
                    .await
                {
                    println!(
                        "[WEBSOCKET] Send error: {}",
                        error
                    );

                    break;
                }
            }

            Ok(Message::Binary(data)) => {
                println!(
                    "[WEBSOCKET] Binary message received: {} bytes",
                    data.len()
                );
            }

            Ok(Message::Ping(data)) => {
                println!(
                    "[WEBSOCKET] Ping received: {} bytes",
                    data.len()
                );
            }

            Ok(Message::Pong(data)) => {
                println!(
                    "[WEBSOCKET] Pong received: {} bytes",
                    data.len()
                );
            }

            Ok(Message::Close(frame)) => {
                println!(
                    "[WEBSOCKET] Controller disconnected: {:?}",
                    frame
                );

                break;
            }

            Err(error) => {
                println!(
                    "[WEBSOCKET] Connection error: {}",
                    error
                );

                break;
            }
        }
    }

    println!("[WEBSOCKET] Connection closed.");
}
