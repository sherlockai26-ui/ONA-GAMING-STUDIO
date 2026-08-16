// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Importamos los módulos necesarios de ona_core
mod display_manager;
mod game_handoff;

use ona_core::game_manager::{
    catalog::GameCatalog,
    importer::{install_or_replace_game_from_dir, GameInstallAction},
    library::GameLibrary,
    profile::GameProfile,
    scanner::{scan_external_game_packages, PackageScanReport},
};
use ona_core::input::{
    bridge::{GameInputBridge, GameInputBridgeStatus},
    dispatcher::dispatch,
    events::normalize_controller_json,
    profile::OnaControllerProfile,
};
use ona_core::launcher::{
    process::{GameLauncher, OnaGameRuntimeContext},
    state::RunningGameStatus,
};
use ona_core::qr::generator::{generate, generate_svg};
use ona_core::runtime::{
    bridge::{GameLifecycleBridge, GameLifecycleBridgeStatus, GameRuntimeSignal},
    lifecycle::GameLifecycleState,
};
use ona_core::session::manager::create_session;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::Mutex, thread, time::Duration};
use tauri::{Emitter, Manager};

struct OnaGameRuntime {
    launcher: GameLauncher,
    input_bridge: Mutex<GameInputBridge>,
    lifecycle_bridge: Mutex<GameLifecycleBridge>,
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
    runtime: tauri::State<'_, OnaGameRuntime>,
    source_dir: String,
    action: Option<GameInstallAction>,
) -> Result<GameProfile, String> {
    let source_dir = PathBuf::from(source_dir);
    let library = game_library(&app_handle)?;
    let action = action.unwrap_or(GameInstallAction::Install);
    let manifest = ona_core::game_manager::manifest::GameManifest::read_from_dir(&source_dir)
        .map_err(|error| error.to_string())?;
    let status = runtime.launcher.status();

    if game_id_is_running(&status, &manifest.identity.id) {
        return Err("GAME_IS_CURRENTLY_RUNNING".to_string());
    }

    install_or_replace_game_from_dir(&library, source_dir, action)
        .map_err(|error| error.to_string())
}

fn game_id_is_running(status: &RunningGameStatus, game_id: &str) -> bool {
    status.state == GameLifecycleState::Running && status.game_id.as_deref() == Some(game_id)
}

#[tauri::command]
fn scan_game_installation_sources(
    app_handle: tauri::AppHandle,
) -> Result<PackageScanReport, String> {
    let library = game_library(&app_handle)?;

    Ok(scan_external_game_packages(&library))
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

    let bridge_status = runtime
        .input_bridge
        .lock()
        .map_err(|error| error.to_string())?
        .status();
    let lifecycle_status = runtime
        .lifecycle_bridge
        .lock()
        .map_err(|error| error.to_string())?
        .status();
    let display_layout =
        display_manager::detect_layout(&app_handle).map_err(|error| error.to_string())?;
    let target_display = display_layout.target_display().ok_or_else(|| {
        "GAMING_DISPLAY_NOT_AVAILABLE: Select another display before launching the game."
            .to_string()
    })?;
    let (display_mode, display_width, display_height) =
        runtime_display_contract_for_profile(&profile, target_display.width, target_display.height);

    runtime
        .lifecycle_bridge
        .lock()
        .map_err(|error| error.to_string())?
        .clear();

    println!(
        "[ONA Runtime] Display contract for {}: ONA_DISPLAY_MODE={} ONA_DISPLAY_ID={} ONA_DISPLAY_X={} ONA_DISPLAY_Y={} ONA_DISPLAY_WIDTH={} ONA_DISPLAY_HEIGHT={} ONA_DISPLAY_SCALE_FACTOR={}",
        profile.id,
        display_mode,
        target_display.identifier,
        target_display.x,
        target_display.y,
        display_width,
        display_height,
        target_display.scale_factor
    );

    runtime.launcher.launch_with_runtime(
        &profile,
        OnaGameRuntimeContext {
            input_host: bridge_status.host,
            input_port: bridge_status.port,
            lifecycle_host: lifecycle_status.host,
            lifecycle_port: lifecycle_status.port,
            player_id: None,
            display_mode,
            display_id: target_display.identifier.clone(),
            display_name: target_display.name.clone(),
            display_x: target_display.x,
            display_y: target_display.y,
            display_width: Some(display_width),
            display_height: Some(display_height),
            display_scale_factor: target_display.scale_factor,
            display_target: Some(target_display.identifier.clone()),
        },
    )
}

fn parse_resolution(resolution: &Option<String>) -> Option<(u32, u32)> {
    let resolution = resolution.as_deref()?;
    let (width, height) = resolution.split_once('x')?;

    Some((width.trim().parse().ok()?, height.trim().parse().ok()?))
}

fn runtime_display_contract_for_profile(
    profile: &GameProfile,
    target_width: u32,
    target_height: u32,
) -> (String, u32, u32) {
    if profile.fullscreen {
        return (
            "CONSOLE_FULLSCREEN".to_string(),
            target_width,
            target_height,
        );
    }

    let (display_width, display_height) =
        parse_resolution(&profile.resolution).unwrap_or((target_width, target_height));

    ("WINDOWED".to_string(), display_width, display_height)
}

#[tauri::command]
fn prepare_shell_for_game(app_handle: tauri::AppHandle) -> Result<(), String> {
    let window = app_handle
        .get_webview_window("main")
        .ok_or_else(|| "Main ONA window was not found.".to_string())?;

    window.hide().map_err(|error| error.to_string())
}

#[tauri::command]
async fn wait_for_game_handoff_ready(
    app_handle: tauri::AppHandle,
    runtime: tauri::State<'_, OnaGameRuntime>,
    pid: u32,
    timeout_ms: u64,
) -> Result<game_handoff::GameHandoffStatus, String> {
    let layout = display_manager::detect_layout(&app_handle).map_err(|error| error.to_string())?;
    let target = layout.target_display().ok_or_else(|| {
        "GAMING_DISPLAY_NOT_AVAILABLE: Select another display before launching the game."
            .to_string()
    })?;
    let launcher_status = runtime.launcher.status();
    let profile = launcher_status
        .game_id
        .as_deref()
        .and_then(|game_id| game_library(&app_handle).ok()?.get_game(game_id).ok());
    let presentation_mode = profile
        .as_ref()
        .map(|profile| runtime_display_contract_for_profile(profile, target.width, target.height).0)
        .unwrap_or_else(|| "CONSOLE_FULLSCREEN".to_string());
    let target = game_handoff::TargetDisplayBounds {
        display_id: target.identifier.clone(),
        x: target.x,
        y: target.y,
        width: target.width,
        height: target.height,
        presentation_mode,
    };
    let lifecycle_bridge = runtime
        .lifecycle_bridge
        .lock()
        .map_err(|error| error.to_string())?
        .clone();

    tauri::async_runtime::spawn_blocking(move || {
        game_handoff::wait_for_game_handoff_ready(
            pid,
            target,
            timeout_ms,
            || lifecycle_bridge.has_signal(GameRuntimeSignal::GameDisplayReady),
            || lifecycle_bridge.has_signal(GameRuntimeSignal::GameReady),
        )
    })
    .await
    .map_err(|error| error.to_string())
}

#[tauri::command]
fn restore_shell_after_game(app_handle: tauri::AppHandle) -> Result<(), String> {
    let window = app_handle
        .get_webview_window("main")
        .ok_or_else(|| "Main ONA window was not found.".to_string())?;

    let _ = window.unminimize();
    window.show().map_err(|error| error.to_string())?;
    let layout = display_manager::detect_layout(&app_handle).map_err(|error| error.to_string())?;
    display_manager::apply_layout_to_window(&window, &layout).map_err(|error| error.to_string())?;
    window.set_focus().map_err(|error| error.to_string())
}

#[tauri::command]
fn running_game_status(
    runtime: tauri::State<'_, OnaGameRuntime>,
) -> Result<RunningGameStatus, String> {
    Ok(runtime.launcher.status())
}

#[tauri::command]
fn uninstall_installed_game(
    app_handle: tauri::AppHandle,
    runtime: tauri::State<'_, OnaGameRuntime>,
    game_id: String,
) -> Result<(), String> {
    let status = runtime.launcher.status();

    if status.state == GameLifecycleState::Running && status.game_id.as_deref() == Some(&game_id) {
        return Err("GAME_IS_CURRENTLY_RUNNING".to_string());
    }

    let library = game_library(&app_handle)?;
    library
        .uninstall_game(&game_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn terminate_running_game(
    runtime: tauri::State<'_, OnaGameRuntime>,
) -> Result<RunningGameStatus, String> {
    if let Ok(bridge) = runtime.lifecycle_bridge.lock() {
        bridge.send_control_signal("ONA_SHUTDOWN");
    }

    thread::sleep(Duration::from_millis(500));

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
fn game_lifecycle_bridge_status(
    runtime: tauri::State<'_, OnaGameRuntime>,
) -> Result<GameLifecycleBridgeStatus, String> {
    let bridge = runtime
        .lifecycle_bridge
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

#[cfg(test)]
mod tests {
    use super::game_id_is_running;
    use ona_core::{launcher::state::RunningGameStatus, runtime::lifecycle::GameLifecycleState};

    #[test]
    fn running_game_blocks_replacement_for_same_game_id_only() {
        let running = RunningGameStatus {
            game_id: Some("studio.test.running".to_string()),
            pid: Some(42),
            state: GameLifecycleState::Running,
            exit_code: None,
            error: None,
        };

        assert!(game_id_is_running(&running, "studio.test.running"));
        assert!(!game_id_is_running(&running, "studio.test.other"));

        let idle = RunningGameStatus {
            state: GameLifecycleState::Idle,
            ..running
        };

        assert!(!game_id_is_running(&idle, "studio.test.running"));
    }
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
            lifecycle_bridge: Mutex::new(
                GameLifecycleBridge::start_localhost(0)
                    .expect("ONA lifecycle bridge could not bind to localhost"),
            ),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_qr_session,
            list_installed_games,
            scan_game_installation_sources,
            import_local_game,
            launch_installed_game,
            wait_for_game_handoff_ready,
            prepare_shell_for_game,
            restore_shell_after_game,
            running_game_status,
            uninstall_installed_game,
            terminate_running_game,
            game_input_bridge_status,
            game_lifecycle_bridge_status,
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
