use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use serde::Serialize;

use super::events::OnaInputEvent;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInputBridgeStatus {
    pub running: bool,
    pub address: String,
    pub connected_clients: usize,
}

#[derive(Debug, Clone)]
pub struct GameInputBridge {
    address: String,
    clients: Arc<Mutex<Vec<TcpStream>>>,
    running: Arc<Mutex<bool>>,
}

impl GameInputBridge {
    pub fn start_localhost(port: u16) -> Result<Self, String> {
        let listener = TcpListener::bind(("127.0.0.1", port)).map_err(|error| error.to_string())?;
        let address = listener
            .local_addr()
            .map_err(|error| error.to_string())?
            .to_string();

        let clients = Arc::new(Mutex::new(Vec::new()));
        let running = Arc::new(Mutex::new(true));
        let thread_clients = Arc::clone(&clients);
        let thread_running = Arc::clone(&running);

        thread::spawn(move || {
            for stream in listener.incoming() {
                if !thread_running
                    .lock()
                    .map(|running| *running)
                    .unwrap_or(false)
                {
                    break;
                }

                match stream {
                    Ok(stream) => {
                        let _ = stream.set_nonblocking(true);
                        if let Ok(mut clients) = thread_clients.lock() {
                            clients.push(stream);
                        }
                    }
                    Err(error) => eprintln!("[ONA Input Bridge] TCP accept error: {error}"),
                }
            }
        });

        Ok(Self {
            address,
            clients,
            running,
        })
    }

    pub fn send(&self, event: OnaInputEvent) {
        let Ok(payload) = serde_json::to_string(&event) else {
            return;
        };

        let message = format!("{payload}\n");

        if let Ok(mut clients) = self.clients.lock() {
            clients.retain_mut(|client| client.write_all(message.as_bytes()).is_ok());
        }
    }

    pub fn status(&self) -> GameInputBridgeStatus {
        GameInputBridgeStatus {
            running: self.running.lock().map(|running| *running).unwrap_or(false),
            address: self.address.clone(),
            connected_clients: self
                .clients
                .lock()
                .map(|clients| clients.len())
                .unwrap_or(0),
        }
    }
}
