use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameRuntimeSignal {
    GameStarted,
    GameWindowReady,
    GameDisplayReady,
    GameReady,
    GameExiting,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameLifecycleBridgeStatus {
    pub running: bool,
    pub address: String,
    pub host: String,
    pub port: u16,
    pub game_started: bool,
    pub game_window_ready: bool,
    pub game_display_ready: bool,
    pub game_ready: bool,
    pub game_exiting: bool,
}

#[derive(Debug, Clone)]
pub struct GameLifecycleBridge {
    address: String,
    host: String,
    port: u16,
    signals: Arc<Mutex<Vec<GameRuntimeSignal>>>,
    clients: Arc<Mutex<Vec<TcpStream>>>,
    running: Arc<Mutex<bool>>,
}

impl GameLifecycleBridge {
    pub fn start_localhost(port: u16) -> Result<Self, String> {
        let listener = TcpListener::bind(("127.0.0.1", port)).map_err(|error| error.to_string())?;
        let local_addr = listener.local_addr().map_err(|error| error.to_string())?;
        let address = local_addr.to_string();
        let host = local_addr.ip().to_string();
        let port = local_addr.port();

        let signals = Arc::new(Mutex::new(Vec::new()));
        let clients = Arc::new(Mutex::new(Vec::new()));
        let running = Arc::new(Mutex::new(true));
        let thread_signals = Arc::clone(&signals);
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
                        if let Ok(client) = stream.try_clone() {
                            if let Ok(mut clients) = thread_clients.lock() {
                                clients.push(client);
                            }
                        }

                        let signals = Arc::clone(&thread_signals);
                        thread::spawn(move || {
                            let reader = BufReader::new(stream);
                            for line in reader.lines().map_while(Result::ok) {
                                if let Some(signal) = parse_runtime_signal(&line) {
                                    if let Ok(mut signals) = signals.lock() {
                                        signals.push(signal);
                                    }
                                }
                            }
                        });
                    }
                    Err(error) => eprintln!("[ONA Runtime Bridge] TCP accept error: {error}"),
                }
            }
        });

        Ok(Self {
            address,
            host,
            port,
            signals,
            clients,
            running,
        })
    }

    pub fn has_signal(&self, signal: GameRuntimeSignal) -> bool {
        self.signals
            .lock()
            .map(|signals| signals.contains(&signal))
            .unwrap_or(false)
    }

    pub fn clear(&self) {
        if let Ok(mut signals) = self.signals.lock() {
            signals.clear();
        }
    }

    pub fn send_control_signal(&self, signal: &str) {
        let message = format!("{signal}\n");

        if let Ok(mut clients) = self.clients.lock() {
            clients.retain_mut(|client| client.write_all(message.as_bytes()).is_ok());
        }
    }

    pub fn status(&self) -> GameLifecycleBridgeStatus {
        GameLifecycleBridgeStatus {
            running: self.running.lock().map(|running| *running).unwrap_or(false),
            address: self.address.clone(),
            host: self.host.clone(),
            port: self.port,
            game_started: self.has_signal(GameRuntimeSignal::GameStarted),
            game_window_ready: self.has_signal(GameRuntimeSignal::GameWindowReady),
            game_display_ready: self.has_signal(GameRuntimeSignal::GameDisplayReady),
            game_ready: self.has_signal(GameRuntimeSignal::GameReady),
            game_exiting: self.has_signal(GameRuntimeSignal::GameExiting),
        }
    }
}

fn parse_runtime_signal(raw: &str) -> Option<GameRuntimeSignal> {
    let trimmed = raw.trim();
    let value = serde_json::from_str::<serde_json::Value>(trimmed)
        .ok()
        .and_then(|json| {
            json.get("event")
                .or_else(|| json.get("type"))
                .and_then(|value| value.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| trimmed.to_string());

    match value.trim().to_ascii_uppercase().as_str() {
        "GAME_STARTED" => Some(GameRuntimeSignal::GameStarted),
        "GAME_WINDOW_READY" => Some(GameRuntimeSignal::GameWindowReady),
        "GAME_DISPLAY_READY" => Some(GameRuntimeSignal::GameDisplayReady),
        "GAME_READY" => Some(GameRuntimeSignal::GameReady),
        "GAME_EXITING" => Some(GameRuntimeSignal::GameExiting),
        _ => None,
    }
}
