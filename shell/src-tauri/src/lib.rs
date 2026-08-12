// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Importamos los módulos necesarios de ona_core
mod display_manager;

use ona_core::game_manager::{
    catalog::GameCatalog, importer::import_game_from_dir, library::GameLibrary,
    profile::GameProfile,
};
use ona_core::input::{
    bridge::{GameInputBridge, GameInputBridgeStatus},
    dispatcher::dispatch,
    events::normalize_controller_json,
    profile::OnaControllerProfile,
};
use ona_core::launcher::{process::GameLauncher, state::RunningGameStatus};
use ona_core::qr::generator::{generate, generate_svg};
use ona_core::session::manager::create_session;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::Mutex};
use tauri::{Emitter, Manager};

struct OnaGameRuntime {
    launcher: GameLauncher,
    input_bridge: Mutex<GameInputBridge>,
}

#[derive(Serialize)]
struct QrSession {
    url: String,
    svg: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct OnaShellSettings {
    language: String,
    ui_animations: bool,
    reduced_motion: bool,
    visual_intensity: String,
    ui_muted: bool,
    ui_volume: u8,
}

impl Default for OnaShellSettings {
    fn default() -> Self {
        Self {
            language: "English".to_string(),
            ui_animations: true,
            reduced_motion: false,
            visual_intensity: "normal".to_string(),
            ui_muted: false,
            ui_volume: 70,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OnaSystemInfo {
    version: String,
    platform: String,
    architecture: String,
    app_data_path: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OnaStorageInfo {
    app_data_path: String,
    installed_games: usize,
    app_data_bytes: u64,
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

fn game_library(app_handle: &tauri::AppHandle) -> Result<GameLibrary, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;

    Ok(GameLibrary::new(app_data_dir))
}

fn settings_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;

    Ok(app_data_dir.join("settings.json"))
}

fn directory_size(path: &std::path::Path) -> u64 {
    let Ok(entries) = fs::read_dir(path) else {
        return 0;
    };

    entries
        .filter_map(Result::ok)
        .map(|entry| {
            let Ok(metadata) = entry.metadata() else {
                return 0;
            };

            if metadata.is_dir() {
                directory_size(&entry.path())
            } else {
                metadata.len()
            }
        })
        .sum()
}

#[tauri::command]
fn load_shell_settings(app_handle: tauri::AppHandle) -> Result<OnaShellSettings, String> {
    let settings_path = settings_path(&app_handle)?;

    if !settings_path.is_file() {
        return Ok(OnaShellSettings::default());
    }

    let settings = fs::read_to_string(settings_path).map_err(|error| error.to_string())?;
    serde_json::from_str::<OnaShellSettings>(&settings).map_err(|error| error.to_string())
}

#[tauri::command]
fn save_shell_settings(
    app_handle: tauri::AppHandle,
    settings: OnaShellSettings,
) -> Result<OnaShellSettings, String> {
    let settings_path = settings_path(&app_handle)?;

    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    let settings_json =
        serde_json::to_string_pretty(&settings).map_err(|error| error.to_string())?;
    fs::write(settings_path, settings_json).map_err(|error| error.to_string())?;

    Ok(settings)
}

#[tauri::command]
fn display_layout(app_handle: tauri::AppHandle) -> Result<display_manager::DisplayLayout, String> {
    display_manager::detect_layout(&app_handle).map_err(|error| error.to_string())
}

#[tauri::command]
fn system_information(app_handle: tauri::AppHandle) -> Result<OnaSystemInfo, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;

    Ok(OnaSystemInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        platform: std::env::consts::OS.to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        app_data_path: app_data_dir.to_string_lossy().to_string(),
    })
}

#[tauri::command]
fn storage_information(app_handle: tauri::AppHandle) -> Result<OnaStorageInfo, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    let games = game_library(&app_handle)?
        .list_games()
        .map_err(|error| error.to_string())?;

    Ok(OnaStorageInfo {
        app_data_path: app_data_dir.to_string_lossy().to_string(),
        installed_games: games.games.len(),
        app_data_bytes: directory_size(&app_data_dir),
    })
}

#[tauri::command]
fn list_installed_games(app_handle: tauri::AppHandle) -> Result<GameCatalog, String> {
    game_library(&app_handle)?
        .list_games()
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn import_local_game(
    app_handle: tauri::AppHandle,
    source_dir: String,
    overwrite: bool,
) -> Result<GameProfile, String> {
    let source_dir = PathBuf::from(source_dir);
    let library = game_library(&app_handle)?;

    import_game_from_dir(&library, source_dir, overwrite).map_err(|error| error.to_string())
}

#[tauri::command]
fn launch_installed_game(
    app_handle: tauri::AppHandle,
    runtime: tauri::State<'_, OnaGameRuntime>,
    game_id: String,
) -> Result<RunningGameStatus, String> {
    let library = game_library(&app_handle)?;
    let profile = library
        .get_game(&game_id)
        .map_err(|error| error.to_string())?;

    runtime.launcher.launch(&profile)
}

#[tauri::command]
fn running_game_status(
    runtime: tauri::State<'_, OnaGameRuntime>,
) -> Result<RunningGameStatus, String> {
    Ok(runtime.launcher.status())
}

#[tauri::command]
fn terminate_running_game(
    runtime: tauri::State<'_, OnaGameRuntime>,
) -> Result<RunningGameStatus, String> {
    runtime.launcher.terminate()
}

#[tauri::command]
fn game_input_bridge_status(
    runtime: tauri::State<'_, OnaGameRuntime>,
) -> Result<GameInputBridgeStatus, String> {
    let bridge = runtime
        .input_bridge
        .lock()
        .map_err(|error| error.to_string())?;

    Ok(bridge.status())
}

#[tauri::command]
fn minimize_main_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    let window = app_handle
        .get_webview_window("main")
        .ok_or_else(|| "Main ONA window was not found.".to_string())?;

    window.minimize().map_err(|error| error.to_string())
}

fn controller_profile_path(
    app_handle: &tauri::AppHandle,
    player_id: u8,
) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;

    Ok(app_data_dir
        .join("controller")
        .join(format!("player-{player_id}-profile.json")))
}

#[tauri::command]
fn load_controller_profile(
    app_handle: tauri::AppHandle,
    player_id: u8,
) -> Result<OnaControllerProfile, String> {
    let profile_path = controller_profile_path(&app_handle, player_id)?;

    if !profile_path.is_file() {
        return Ok(OnaControllerProfile::default());
    }

    let profile = fs::read_to_string(profile_path).map_err(|error| error.to_string())?;
    serde_json::from_str::<OnaControllerProfile>(&profile).map_err(|error| error.to_string())
}

#[tauri::command]
fn save_controller_profile(
    app_handle: tauri::AppHandle,
    player_id: u8,
    profile: OnaControllerProfile,
) -> Result<OnaControllerProfile, String> {
    let profile_path = controller_profile_path(&app_handle, player_id)?;

    if let Some(parent) = profile_path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    let profile_json = serde_json::to_string_pretty(&profile).map_err(|error| error.to_string())?;
    fs::write(profile_path, profile_json).map_err(|error| error.to_string())?;

    Ok(profile)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut controller_connections = ona_core::network::websocket::subscribe_connections();
    let mut controller_inputs = ona_core::network::websocket::subscribe_inputs();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(OnaGameRuntime {
            launcher: GameLauncher::new(),
            input_bridge: Mutex::new(
                GameInputBridge::start_localhost(0)
                    .expect("ONA input bridge could not bind to localhost"),
            ),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_qr_session,
            list_installed_games,
            import_local_game,
            launch_installed_game,
            running_game_status,
            terminate_running_game,
            game_input_bridge_status,
            minimize_main_window,
            load_shell_settings,
            save_shell_settings,
            display_layout,
            system_information,
            storage_information,
            load_controller_profile,
            save_controller_profile
        ])
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
            let runtime_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                while let Ok(input) = controller_inputs.recv().await {
                    if let Some(event) = normalize_controller_json(&input) {
                        if let Some(runtime) = runtime_handle.try_state::<OnaGameRuntime>() {
                            if let Ok(bridge) = runtime.input_bridge.lock() {
                                dispatch(event, Some(&bridge));
                            }
                        }
                    }

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
