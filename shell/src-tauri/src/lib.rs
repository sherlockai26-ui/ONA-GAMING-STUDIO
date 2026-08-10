// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Importamos los módulos necesarios de ona_core
mod display_manager;

use ona_core::qr::generator::{generate, generate_svg};
use ona_core::session::manager::create_session;
use serde::Serialize;
use tauri::Emitter;

#[derive(Serialize)]
struct QrSession {
    url: String,
    svg: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn generate_qr_session() -> Result<QrSession, String> {
    // Creamos una sesión con su token
    let session = create_session();
    // Generamos la URL para el QR (usamos el puerto 8080, puedes cambiarlo)
    let url = generate(&session.id, &session.token.value, 8080);
    let svg = generate_svg(&url, 150, 150)?;

    Ok(QrSession { url, svg })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut controller_connections = ona_core::network::websocket::subscribe_connections();
    let mut controller_inputs = ona_core::network::websocket::subscribe_inputs();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, generate_qr_session])
        .setup(|app| {
            display_manager::configure_main_window(app)?;

            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                while controller_connections.recv().await.is_ok() {
                    if let Err(error) = app_handle.emit("controller-connected", ()) {
                        eprintln!("Could not notify the shell about a controller: {error}");
                    }
                }
            });

            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                while let Ok(input) = controller_inputs.recv().await {
                    if let Err(error) = app_handle.emit("controller-input", input) {
                        eprintln!("Could not forward controller input to the shell: {error}");
                    }
                }
            });

            tauri::async_runtime::spawn(async {
                if let Err(error) = ona_core::network::http_server::start().await {
                    eprintln!("ONA Core HTTP server failed: {error}");
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
