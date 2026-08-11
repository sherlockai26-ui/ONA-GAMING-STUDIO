// main.rs
// Punto de entrada principal de ONA Core

mod network {
    pub mod discovery;
    pub mod http_server;
    pub mod websocket;
}

mod session {
    pub mod manager;
    pub mod token;
}

mod qr {
    pub mod generator;
}

mod input {
    pub mod bridge;
    pub mod calibration;
    pub mod dispatcher;
    pub mod events;
    pub mod player_event;
    pub mod profile;
}

mod controller {
    pub mod manager;
}

mod events {
    pub mod bus;
}

mod game_manager {
    pub mod catalog;
    pub mod errors;
    pub mod importer;
    pub mod library;
    pub mod manifest;
    pub mod profile;
    pub mod storage;
}

mod launcher {
    pub mod process;
    pub mod state;
}

mod runtime {
    pub mod lifecycle;
}

mod security {
    pub mod session_guard;
}

mod storage {
    pub mod config;
}

use qr::generator::generate;
use session::manager::create_session;
use storage::config::CoreConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("======================================");
    println!("        ONA CORE INITIALIZING");
    println!("======================================");

    println!("[1/6] Loading configuration...");

    let config = CoreConfig::default();

    println!(
        "Port: {} | Max Players: {}",
        config.port, config.max_players
    );

    println!("[2/6] Detecting local network...");

    network::discovery::initialize();

    println!("[3/6] Creating controller session...");

    let session = create_session();

    println!("[4/6] Generating QR connection URL...");

    let url = generate(&session.id, &session.token.value, config.port);

    println!("Controller URL:");
    println!("{}", url);

    println!("[5/6] Starting HTTP server...");

    network::http_server::start().await?;

    println!("[6/6] ONA CORE READY");

    Ok(())
}
