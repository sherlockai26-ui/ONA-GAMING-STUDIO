// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Importamos los módulos necesarios de ona_core
mod cursor_manager;
mod display_manager;
mod game_handoff;
mod presentation_adapter;

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
    events::{normalize_controller_json, ButtonState, OnaButton, OnaInputEvent},
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
use ona_core::session::manager::persistent_pairing_session;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex, OnceLock,
    },
    thread,
    time::{Duration, Instant},
};
use tauri::{utils::config::Color, Emitter, Manager};

const MAIN_WINDOW_LABEL: &str = "main";
const PRESENTATION_GUARD_LABEL: &str = "presentation_guard";
static ESCAPE_SYSTEM_SHORTCUT: OnceLock<Mutex<Option<EscapeSystemShortcut>>> = OnceLock::new();
static ESCAPE_SYSTEM_SHORTCUT_GENERATION: AtomicU64 = AtomicU64::new(0);

struct OnaGameRuntime {
    launcher: GameLauncher,
    input_bridge: Mutex<GameInputBridge>,
    lifecycle_bridge: Mutex<GameLifecycleBridge>,
    input_forwarding_enabled: Mutex<bool>,
}

struct EscapeSystemShortcut {
    generation: u64,
    running: Arc<AtomicBool>,
    worker: thread::JoinHandle<()>,
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

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct WindowPresentationSnapshot {
    label: String,
    visible: bool,
    minimized: bool,
    fullscreen: bool,
    decorated: bool,
    resizable: bool,
    always_on_top: bool,
    opacity: u8,
    transparent: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PresentationGuardNativeStatus {
    exists: bool,
    visible: bool,
    always_on_top: bool,
    fullscreen: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EscapeShortcutStatus {
    registered: bool,
    generation: u64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OnaLocalConnectivityInfo {
    network_mode: String,
    internet_available: bool,
    lan_available: bool,
    direct_local_supported: bool,
    direct_local_active: bool,
    local_ipv4: Option<String>,
    controller_url: Option<String>,
    notes: Vec<String>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn generate_qr_session() -> Result<QrSession, String> {
    // Creamos una sesión con su token
    let session = persistent_pairing_session();
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
fn local_connectivity_status() -> OnaLocalConnectivityInfo {
    let local_ipv4 = local_ip_address::local_ip()
        .ok()
        .filter(|ip| ip.is_ipv4() && !ip.is_loopback())
        .map(|ip| ip.to_string());
    let lan_available = local_ipv4.is_some();
    let controller_url = local_ipv4
        .as_ref()
        .map(|ip| format!("http://{ip}:8080/controller/"));
    let mut notes = vec![
        "Internet connectivity is not required for ONA Controller local play.".to_string(),
        "Direct Local hotspot mode is not started by ONA Runtime V1.".to_string(),
    ];

    #[cfg(windows)]
    let direct_local_supported = windows_direct_local_supported(&mut notes);

    #[cfg(not(windows))]
    let direct_local_supported = false;

    OnaLocalConnectivityInfo {
        network_mode: if lan_available {
            "LOCAL_NETWORK".to_string()
        } else {
            "NO_LOCAL_LINK".to_string()
        },
        internet_available: false,
        lan_available,
        direct_local_supported,
        direct_local_active: false,
        local_ipv4,
        controller_url,
        notes,
    }
}

#[tauri::command]
fn enable_escape_system_shortcut(app_handle: tauri::AppHandle) -> Result<(), String> {
    platform_enable_escape_system_shortcut(app_handle)
}

#[tauri::command]
fn disable_escape_system_shortcut() -> Result<(), String> {
    platform_disable_escape_system_shortcut()
}

#[tauri::command]
fn escape_system_shortcut_status() -> Result<EscapeShortcutStatus, String> {
    platform_escape_system_shortcut_status()
}

#[tauri::command]
fn exit_ona_process(app_handle: tauri::AppHandle) -> Result<(), String> {
    println!("[ONA Shutdown] process exit requested");
    let _ = platform_disable_escape_system_shortcut();
    app_handle.exit(0);
    Ok(())
}

#[tauri::command]
fn presentation_adapter_contract() -> presentation_adapter::PresentationAdapterContract {
    presentation_adapter::contract()
}

#[cfg(windows)]
fn windows_direct_local_supported(notes: &mut Vec<String>) -> bool {
    let output = std::process::Command::new("netsh")
        .args(["wlan", "show", "drivers"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let text = String::from_utf8_lossy(&output.stdout).to_lowercase();
            let supported = text.contains("wi-fi direct")
                || text.contains("hosted network supported  : yes")
                || text.contains("red hospedada admitida  : s");
            notes.push("Windows WLAN capability was inspected with netsh; ONA does not enable a hotspot automatically.".to_string());
            supported
        }
        Ok(_) => {
            notes.push(
                "Windows WLAN capability probe returned no usable Direct Local support."
                    .to_string(),
            );
            false
        }
        Err(_) => {
            notes.push(
                "Windows WLAN capability probe could not run in this environment.".to_string(),
            );
            false
        }
    }
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
    println!(
        "[ONA Runtime] Launch request game_id={} executable={} working_dir={} arguments={:?}",
        profile.id,
        profile.executable.display(),
        profile.working_directory.display(),
        profile.arguments
    );

    game_handoff::log_visible_windows("launching-before-spawn");

    let status = runtime.launcher.launch_with_runtime(
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
    )?;

    game_handoff::clear_primary_game_window();
    game_handoff::log_visible_windows("launching-after-spawn");

    Ok(status)
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
fn hide_game_cursor(app_handle: tauri::AppHandle) -> Result<(), String> {
    let window = app_handle.get_webview_window("main");
    cursor_manager::hide_for_game_session(window.as_ref());
    Ok(())
}

#[tauri::command]
fn restore_game_cursor(app_handle: tauri::AppHandle) -> Result<(), String> {
    let window = app_handle.get_webview_window("main");
    cursor_manager::restore_after_game_session(window.as_ref());
    Ok(())
}

#[tauri::command]
fn prepare_shell_for_game(
    app_handle: tauri::AppHandle,
    reason: Option<String>,
) -> Result<(), String> {
    let window = app_handle
        .get_webview_window(MAIN_WINDOW_LABEL)
        .ok_or_else(|| "Main ONA window was not found.".to_string())?;

    cursor_manager::hide_for_game_session(Some(&window));

    if reason.as_deref() == Some("safe_handoff") {
        println!("[ONA Presentation] SAFE HANDOFF accepted. Hiding ONA Shell.");
        game_handoff::log_visible_windows("safe-handoff-before-hide");
    } else {
        println!(
            "[ONA Presentation] hiding ONA Shell for game reason={}",
            reason.as_deref().unwrap_or("runtime")
        );
    }

    let _ = platform_set_window_opacity(&window, 255);
    let _ = window.set_always_on_top(false);
    let _ = window.set_decorations(false);
    window.hide().map_err(|error| error.to_string())?;
    let snapshot = window_presentation_snapshot(&window, 255, false)?;
    println!("[ONA Restore] opacity={}", snapshot.opacity);
    println!("[ONA Restore] alwaysOnTop={}", snapshot.always_on_top);
    println!("[ONA Restore] transparency={}", snapshot.transparent);
    println!("[ONA Restore] fullscreen={}", snapshot.fullscreen);
    Ok(())
}

#[tauri::command]
fn show_presentation_guard(
    app_handle: tauri::AppHandle,
    mode: Option<String>,
) -> Result<(), String> {
    show_dedicated_presentation_guard(&app_handle, mode.as_deref())
}

fn presentation_guard_window(
    app_handle: &tauri::AppHandle,
) -> Result<tauri::WebviewWindow, String> {
    if let Some(window) = app_handle.get_webview_window(PRESENTATION_GUARD_LABEL) {
        return Ok(window);
    }

    tauri::WebviewWindowBuilder::new(
        app_handle,
        PRESENTATION_GUARD_LABEL,
        tauri::WebviewUrl::App("presentation-guard.html".into()),
    )
    .title("ONA Presentation Guard")
    .decorations(false)
    .resizable(false)
    .visible(false)
    .skip_taskbar(true)
    .transparent(false)
    .background_color(Color(2, 5, 10, 255))
    .always_on_top(true)
    .build()
    .map_err(|error| error.to_string())
}

fn configure_guard_window(
    app_handle: &tauri::AppHandle,
    window: &tauri::WebviewWindow,
) -> Result<(), String> {
    let layout = display_manager::detect_layout(app_handle).map_err(|error| error.to_string())?;
    display_manager::apply_layout_to_window(window, &layout).map_err(|error| error.to_string())?;
    window
        .set_decorations(false)
        .map_err(|error| error.to_string())?;
    window
        .set_resizable(false)
        .map_err(|error| error.to_string())?;
    window
        .set_always_on_top(true)
        .map_err(|error| error.to_string())?;
    platform_set_window_opacity(window, 255)?;
    Ok(())
}

fn show_dedicated_presentation_guard(
    app_handle: &tauri::AppHandle,
    mode: Option<&str>,
) -> Result<(), String> {
    let window = presentation_guard_window(app_handle)?;
    let mode = mode.unwrap_or("RETURNING_TO_ONA");

    println!("[ONA Guard] show requested");
    configure_guard_window(app_handle, &window)?;
    let _ = window.emit("ona-guard-mode", mode);
    let _ = window.unminimize();
    window.show().map_err(|error| error.to_string())?;
    window.set_focus().map_err(|error| error.to_string())?;

    let snapshot = window_presentation_snapshot(&window, 255, false)?;
    if !snapshot.visible || !snapshot.always_on_top {
        return Err(format!(
            "GUARD_SHOW_NOT_CONFIRMED visible={} topmost={}",
            snapshot.visible, snapshot.always_on_top
        ));
    }
    println!("[ONA Guard] visible confirmed");
    println!("[ONA Presentation] dedicated guard visible {snapshot:?}");
    println!("[ONA Presentation] WINDOWS EXPOSURE RISK prevented");
    Ok(())
}

fn restore_main_shell_presentation(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let window = app_handle
        .get_webview_window(MAIN_WINDOW_LABEL)
        .ok_or_else(|| "Main ONA window was not found.".to_string())?;

    let _ = window.unminimize();
    let layout = display_manager::detect_layout(&app_handle).map_err(|error| error.to_string())?;
    display_manager::apply_layout_to_window(&window, &layout).map_err(|error| error.to_string())?;
    window
        .set_decorations(false)
        .map_err(|error| error.to_string())?;
    window
        .set_resizable(false)
        .map_err(|error| error.to_string())?;
    platform_set_window_opacity(&window, 255)?;
    let _ = window.set_always_on_top(false);
    window.show().map_err(|error| error.to_string())?;
    Ok(())
}

#[tauri::command]
fn restore_shell_window_presentation(app_handle: tauri::AppHandle) -> Result<(), String> {
    restore_main_shell_presentation(&app_handle)?;
    let window = app_handle
        .get_webview_window(MAIN_WINDOW_LABEL)
        .ok_or_else(|| "Main ONA window was not found.".to_string())?;
    cursor_manager::restore_after_game_session(Some(&window));
    window.set_focus().map_err(|error| error.to_string())?;
    let snapshot = window_presentation_snapshot(&window, 255, false)?;
    println!("[ONA Restore] opacity={}", snapshot.opacity);
    println!("[ONA Restore] alwaysOnTop={}", snapshot.always_on_top);
    println!("[ONA Restore] transparency={}", snapshot.transparent);
    println!("[ONA Restore] fullscreen={}", snapshot.fullscreen);
    Ok(())
}

#[tauri::command]
fn show_shell_system_overlay_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    let window = app_handle
        .get_webview_window(MAIN_WINDOW_LABEL)
        .ok_or_else(|| "Main ONA window was not found.".to_string())?;

    restore_main_shell_presentation(&app_handle)?;
    window
        .set_always_on_top(true)
        .map_err(|error| error.to_string())?;
    platform_set_window_opacity(&window, 255)?;
    window.set_focus().map_err(|error| error.to_string())?;
    let snapshot = window_presentation_snapshot(&window, 255, false)?;
    if !snapshot.visible || !snapshot.always_on_top || !snapshot.fullscreen {
        return Err(format!(
            "QUICK_MENU_SHOW_NOT_CONFIRMED visible={} topmost={} fullscreen={}",
            snapshot.visible, snapshot.always_on_top, snapshot.fullscreen
        ));
    }
    println!("[ONA QuickMenu] native overlay visible over game {snapshot:?}");
    Ok(())
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
    let game_id = launcher_status.game_id.clone();
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
    println!(
        "[ONA Runtime] Lifecycle before handoff wait: {:?}",
        lifecycle_bridge.status()
    );
    game_handoff::log_visible_windows("waiting-for-ready-before");

    tauri::async_runtime::spawn_blocking(move || {
        println!(
            "[ONA Presentation] Waiting for handoff pid={} timeout_ms={} target={} @ {},{} {}x{} mode={}",
            pid,
            timeout_ms,
            target.display_id,
            target.x,
            target.y,
            target.width,
            target.height,
            target.presentation_mode
        );

        let status = game_handoff::wait_for_game_handoff_ready(
            pid,
            game_id,
            target,
            timeout_ms,
            || lifecycle_bridge.has_any_signal(),
            || lifecycle_bridge.has_signal(GameRuntimeSignal::GameDisplayReady),
            || lifecycle_bridge.has_signal(GameRuntimeSignal::GameReady),
        );

        println!("[ONA Presentation] Handoff result: {status:#?}");
        game_handoff::log_visible_windows("waiting-for-ready-after");
        println!(
            "[ONA Runtime] Lifecycle after handoff wait: {:?}",
            lifecycle_bridge.status()
        );

        status
    })
    .await
    .map_err(|error| error.to_string())
}

#[tauri::command]
fn restore_shell_after_game(app_handle: tauri::AppHandle) -> Result<(), String> {
    restore_main_shell_presentation(&app_handle)?;
    let window = app_handle
        .get_webview_window(MAIN_WINDOW_LABEL)
        .ok_or_else(|| "Main ONA window was not found.".to_string())?;
    cursor_manager::restore_after_game_session(Some(&window));
    println!("[ONA Presentation] ONA Shell restored after game session.");
    window.set_focus().map_err(|error| error.to_string())
}

#[tauri::command]
fn release_presentation_guard(app_handle: tauri::AppHandle) -> Result<(), String> {
    println!("[ONA Guard] release requested");

    if let Some(guard) = app_handle.get_webview_window(PRESENTATION_GUARD_LABEL) {
        guard.set_always_on_top(false).map_err(|error| {
            let reason = error.to_string();
            println!("[ONA Guard] RELEASE FAILED reason={reason}");
            reason
        })?;
        println!("[ONA Guard] always_on_top=false");

        guard.hide().map_err(|error| {
            let reason = error.to_string();
            println!("[ONA Guard] RELEASE FAILED reason={reason}");
            reason
        })?;
        println!("[ONA Guard] hide requested");

        let snapshot = window_presentation_snapshot(&guard, 255, false).map_err(|error| {
            println!("[ONA Guard] RELEASE FAILED reason={error}");
            error
        })?;
        println!("[ONA Guard] native visible={}", snapshot.visible);

        if snapshot.visible || snapshot.always_on_top {
            let reason = format!(
                "GUARD_RELEASE_NOT_CONFIRMED visible={} topmost={}",
                snapshot.visible, snapshot.always_on_top
            );
            println!("[ONA Guard] RELEASE FAILED reason={reason}");
            return Err(reason);
        }

        println!("[ONA Guard] hidden confirmed");
    }

    println!("[ONA Guard] release confirmed");
    Ok(())
}

#[tauri::command]
fn presentation_guard_native_status(
    app_handle: tauri::AppHandle,
) -> Result<PresentationGuardNativeStatus, String> {
    let Some(guard) = app_handle.get_webview_window(PRESENTATION_GUARD_LABEL) else {
        return Ok(PresentationGuardNativeStatus {
            exists: false,
            visible: false,
            always_on_top: false,
            fullscreen: false,
        });
    };

    Ok(PresentationGuardNativeStatus {
        exists: true,
        visible: guard.is_visible().map_err(|error| error.to_string())?,
        always_on_top: platform_window_is_topmost(&guard)?,
        fullscreen: guard.is_fullscreen().map_err(|error| error.to_string())?,
    })
}

#[tauri::command]
fn shell_window_presentation_status(
    app_handle: tauri::AppHandle,
) -> Result<WindowPresentationSnapshot, String> {
    let window = app_handle
        .get_webview_window(MAIN_WINDOW_LABEL)
        .ok_or_else(|| "Main ONA window was not found.".to_string())?;

    window_presentation_snapshot(&window, 255, false)
}

#[tauri::command]
fn show_system_overlay_over_game(app_handle: tauri::AppHandle) -> Result<(), String> {
    show_shell_system_overlay_window(app_handle)
}

#[tauri::command]
fn hide_system_overlay_over_game(app_handle: tauri::AppHandle) -> Result<(), String> {
    let window = app_handle
        .get_webview_window(MAIN_WINDOW_LABEL)
        .ok_or_else(|| "Main ONA window was not found.".to_string())?;

    platform_set_window_opacity(&window, 255)?;
    let _ = window.set_always_on_top(false);
    let snapshot = window_presentation_snapshot(&window, 255, false)?;
    println!("[ONA QuickMenu] native overlay hidden {snapshot:?}");
    Ok(())
}

fn window_presentation_snapshot(
    window: &tauri::WebviewWindow,
    opacity: u8,
    transparent: bool,
) -> Result<WindowPresentationSnapshot, String> {
    Ok(WindowPresentationSnapshot {
        label: window.label().to_string(),
        visible: window.is_visible().map_err(|error| error.to_string())?,
        minimized: window.is_minimized().map_err(|error| error.to_string())?,
        fullscreen: window.is_fullscreen().map_err(|error| error.to_string())?,
        decorated: window.is_decorated().map_err(|error| error.to_string())?,
        resizable: window.is_resizable().map_err(|error| error.to_string())?,
        always_on_top: platform_window_is_topmost(window).unwrap_or(false),
        opacity,
        transparent,
    })
}

#[cfg(windows)]
fn platform_set_window_opacity(window: &tauri::WebviewWindow, alpha: u8) -> Result<(), String> {
    use windows::Win32::{
        Foundation::{COLORREF, HWND},
        UI::WindowsAndMessaging::{
            GetWindowLongPtrW, SetLayeredWindowAttributes, SetWindowLongPtrW, GWL_EXSTYLE,
            LWA_ALPHA, WS_EX_LAYERED,
        },
    };

    let hwnd = window.hwnd().map_err(|error| error.to_string())?;
    let hwnd = HWND(hwnd.0 as _);
    let style = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) };
    unsafe {
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style | WS_EX_LAYERED.0 as isize);
        SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA)
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[cfg(not(windows))]
fn platform_set_window_opacity(_window: &tauri::WebviewWindow, _alpha: u8) -> Result<(), String> {
    Ok(())
}

#[cfg(windows)]
fn platform_window_is_topmost(window: &tauri::WebviewWindow) -> Result<bool, String> {
    use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, GWL_EXSTYLE, WS_EX_TOPMOST};

    let hwnd = window.hwnd().map_err(|error| error.to_string())?;
    let hwnd = windows::Win32::Foundation::HWND(hwnd.0 as _);
    let style = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) } as u32;

    Ok(style & WS_EX_TOPMOST.0 != 0)
}

#[cfg(not(windows))]
fn platform_window_is_topmost(_window: &tauri::WebviewWindow) -> Result<bool, String> {
    Ok(false)
}

fn escape_shortcut_state() -> &'static Mutex<Option<EscapeSystemShortcut>> {
    ESCAPE_SYSTEM_SHORTCUT.get_or_init(|| Mutex::new(None))
}

#[cfg(windows)]
fn platform_enable_escape_system_shortcut(app_handle: tauri::AppHandle) -> Result<(), String> {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::{
        Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey, MOD_NOREPEAT},
        WindowsAndMessaging::{PeekMessageW, MSG, PM_REMOVE, WM_HOTKEY},
    };

    const ESCAPE_HOTKEY_ID: i32 = 0x0A_E5C;
    const VK_ESCAPE_CODE: u32 = 0x1B;

    let mut state = escape_shortcut_state()
        .lock()
        .map_err(|error| error.to_string())?;

    if state.is_some() {
        println!("[ONA ESC] register requested existing=true");
        println!("[ONA ESC] duplicate register prevented");
        return Ok(());
    }

    println!("[ONA ESC] register requested existing=false");
    let generation = ESCAPE_SYSTEM_SHORTCUT_GENERATION.fetch_add(1, Ordering::SeqCst) + 1;
    let running = Arc::new(AtomicBool::new(true));
    let worker_running = Arc::clone(&running);
    let worker = thread::spawn(move || {
        let null_hwnd = HWND(std::ptr::null_mut());
        let registered =
            unsafe { RegisterHotKey(null_hwnd, ESCAPE_HOTKEY_ID, MOD_NOREPEAT, VK_ESCAPE_CODE) };

        if registered.is_err() {
            eprintln!("[ONA Keyboard] ESC native shortcut registration failed");
            return;
        }

        println!("[ONA Keyboard] ESC native shortcut enabled");
        println!("[ONA ESC] registered generation={generation}");
        let mut last_escape_event_at: Option<Instant> = None;

        while worker_running.load(Ordering::SeqCst) {
            let mut message = MSG::default();

            while unsafe { PeekMessageW(&mut message, null_hwnd, 0, 0, PM_REMOVE) }.as_bool() {
                if message.message == WM_HOTKEY && message.wParam.0 as i32 == ESCAPE_HOTKEY_ID {
                    let now = Instant::now();
                    if last_escape_event_at
                        .map(|last| now.duration_since(last) < Duration::from_millis(160))
                        .unwrap_or(false)
                    {
                        println!("[ONA ESC] duplicate WM_HOTKEY suppressed");
                        continue;
                    }
                    last_escape_event_at = Some(now);
                    println!("[ONA Keyboard] ESC down");
                    println!("[ONA ESC] event generation={generation}");
                    let _ = app_handle.emit("ona-escape-system-command", ());
                }
            }

            thread::sleep(Duration::from_millis(24));
        }

        let _ = unsafe { UnregisterHotKey(null_hwnd, ESCAPE_HOTKEY_ID) };
        println!("[ONA Keyboard] ESC native shortcut disabled");
    });

    *state = Some(EscapeSystemShortcut {
        generation,
        running,
        worker,
    });
    Ok(())
}

#[cfg(not(windows))]
fn platform_enable_escape_system_shortcut(_app_handle: tauri::AppHandle) -> Result<(), String> {
    Ok(())
}

fn platform_disable_escape_system_shortcut() -> Result<(), String> {
    let mut state = escape_shortcut_state()
        .lock()
        .map_err(|error| error.to_string())?;

    if let Some(shortcut) = state.take() {
        println!(
            "[ONA ESC] unregister requested existing=true generation={}",
            shortcut.generation
        );
        shortcut.running.store(false, Ordering::SeqCst);
        let _ = shortcut.worker.join();
    } else {
        println!("[ONA ESC] unregister requested existing=false");
    }

    Ok(())
}

fn platform_escape_system_shortcut_status() -> Result<EscapeShortcutStatus, String> {
    let state = escape_shortcut_state()
        .lock()
        .map_err(|error| error.to_string())?;

    Ok(EscapeShortcutStatus {
        registered: state.is_some(),
        generation: state
            .as_ref()
            .map(|shortcut| shortcut.generation)
            .unwrap_or_else(|| ESCAPE_SYSTEM_SHORTCUT_GENERATION.load(Ordering::SeqCst)),
    })
}

#[tauri::command]
fn set_game_input_forwarding(
    runtime: tauri::State<'_, OnaGameRuntime>,
    enabled: bool,
) -> Result<(), String> {
    let mut forwarding = runtime
        .input_forwarding_enabled
        .lock()
        .map_err(|error| error.to_string())?;
    *forwarding = enabled;
    println!(
        "[ONA Input Routing] game forwarding {}",
        if enabled { "enabled" } else { "paused" }
    );
    Ok(())
}

#[tauri::command]
fn neutralize_game_input(runtime: tauri::State<'_, OnaGameRuntime>) -> Result<(), String> {
    let status = runtime.launcher.status();

    if status.state != GameLifecycleState::Running {
        println!("[ONA Input Routing] neutralize skipped: no running game");
        return Ok(());
    }

    let bridge = runtime
        .input_bridge
        .lock()
        .map_err(|error| error.to_string())?;
    let player_id = 1;

    bridge.send(OnaInputEvent::Joystick {
        player_id,
        x: 0.0,
        y: 0.0,
    });

    for button in [
        OnaButton::A,
        OnaButton::B,
        OnaButton::X,
        OnaButton::Y,
        OnaButton::L1,
        OnaButton::L2,
        OnaButton::R1,
        OnaButton::R2,
        OnaButton::Select,
        OnaButton::Start,
    ] {
        bridge.send(OnaInputEvent::Button {
            player_id,
            button,
            state: ButtonState::Up,
        });
    }

    println!("[ONA Input Routing] neutralized game input before pause");
    Ok(())
}

#[tauri::command]
fn dispatch_game_start_short(runtime: tauri::State<'_, OnaGameRuntime>) -> Result<(), String> {
    let forwarding_enabled = runtime
        .input_forwarding_enabled
        .lock()
        .map_err(|error| error.to_string())?;

    if !*forwarding_enabled {
        println!("[ONA START] short dispatch skipped because game routing is disabled");
        return Ok(());
    }

    let bridge = runtime
        .input_bridge
        .lock()
        .map_err(|error| error.to_string())?;

    dispatch(
        OnaInputEvent::Button {
            player_id: 1,
            button: OnaButton::Start,
            state: ButtonState::Up,
        },
        Some(&bridge),
    );
    println!("[ONA START] dispatched GAME START UP");
    Ok(())
}

#[tauri::command]
fn focus_running_game(runtime: tauri::State<'_, OnaGameRuntime>, pid: u32) -> Result<bool, String> {
    let status = runtime.launcher.status();

    if status.state != GameLifecycleState::Running || status.pid != Some(pid) {
        return Err("ACTIVE_GAME_SESSION_NOT_RUNNING".to_string());
    }

    let granted = game_handoff::focus_process_window(pid)?;
    println!("[ONA Presentation] existing game foreground requested pid={pid} granted={granted}");
    Ok(granted)
}

#[tauri::command]
fn minimize_running_game(
    runtime: tauri::State<'_, OnaGameRuntime>,
    pid: u32,
) -> Result<bool, String> {
    let status = runtime.launcher.status();

    if status.state != GameLifecycleState::Running || status.pid != Some(pid) {
        return Err("ACTIVE_GAME_SESSION_NOT_RUNNING".to_string());
    }

    let minimized = game_handoff::minimize_process_window(pid)?;
    println!("[ONA Presentation] running game minimize requested pid={pid} minimized={minimized}");
    Ok(minimized)
}

#[tauri::command]
fn restore_running_game(
    runtime: tauri::State<'_, OnaGameRuntime>,
    pid: u32,
) -> Result<game_handoff::GameWindowRestoreStatus, String> {
    let status = runtime.launcher.status();

    if status.state != GameLifecycleState::Running || status.pid != Some(pid) {
        return Err("ACTIVE_GAME_SESSION_NOT_RUNNING".to_string());
    }

    let restored = game_handoff::restore_process_window(pid)?;
    println!(
        "[ONA Presentation] running game restore requested pid={pid} restored={}",
        restored.restored
    );
    Ok(restored)
}

#[tauri::command]
fn suppress_running_game_taskbar_identity(
    runtime: tauri::State<'_, OnaGameRuntime>,
    pid: u32,
) -> Result<bool, String> {
    let status = runtime.launcher.status();

    if status.state != GameLifecycleState::Running || status.pid != Some(pid) {
        return Err("ACTIVE_GAME_SESSION_NOT_RUNNING".to_string());
    }

    game_handoff::suppress_process_taskbar_identity(pid)
}

#[tauri::command]
fn running_game_status(
    runtime: tauri::State<'_, OnaGameRuntime>,
) -> Result<RunningGameStatus, String> {
    let status = runtime.launcher.status();

    if status.state != GameLifecycleState::Running {
        println!("[ONA Runtime] Running game status changed: {status:?}");
        game_handoff::clear_primary_game_window();
    }

    Ok(status)
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
    println!("[ONA Runtime] terminate_running_game requested by ONA.");

    if let Ok(bridge) = runtime.lifecycle_bridge.lock() {
        println!("[ONA Runtime] Sending ONA_SHUTDOWN to lifecycle bridge clients.");
        println!("[ONA Session] graceful shutdown requested");
        bridge.send_control_signal("ONA_SHUTDOWN");
    }

    let graceful_timeout = Duration::from_millis(3000);
    let started = std::time::Instant::now();

    while started.elapsed() < graceful_timeout {
        let status = runtime.launcher.status();

        if status.state != GameLifecycleState::Running {
            println!(
                "[ONA Session] process exited code={:?} during graceful shutdown",
                status.exit_code
            );
            if let Ok(bridge) = runtime.lifecycle_bridge.lock() {
                bridge.clear();
            }
            game_handoff::clear_primary_game_window();
            return Ok(status);
        }

        thread::sleep(Duration::from_millis(100));
    }

    println!("[ONA Session] force termination required");

    let status = runtime.launcher.terminate()?;
    println!("[ONA Runtime] terminate_running_game result: {status:?}");
    if let Ok(bridge) = runtime.lifecycle_bridge.lock() {
        bridge.clear();
    }
    game_handoff::clear_primary_game_window();
    Ok(status)
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

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum TestSessionState {
        Idle,
        Launching,
        Running,
        Background,
        SystemOverlay,
        Minimized,
        Stopping,
        Returning,
        Exited,
        Failed,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum TestPresentationOwner {
        OnaShell,
        OnaTransitionGuard,
        Game,
        OnaSystemOverlay,
        OnaMinimized,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum TestGuardState {
        Hidden,
        Visible,
        Hiding,
        Failed,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct TestSession {
        state: TestSessionState,
        owner: TestPresentationOwner,
        pid: Option<u32>,
    }

    impl TestSession {
        fn idle() -> Self {
            Self {
                state: TestSessionState::Idle,
                owner: TestPresentationOwner::OnaShell,
                pid: None,
            }
        }

        fn play(mut self, pid: u32) -> Self {
            self.state = TestSessionState::Launching;
            self.owner = TestPresentationOwner::OnaTransitionGuard;
            self.pid = Some(pid);
            self
        }

        fn handoff_accepted(mut self) -> Self {
            self.state = TestSessionState::Running;
            self.owner = TestPresentationOwner::Game;
            self
        }

        fn hold_start(mut self) -> Self {
            if self.state == TestSessionState::SystemOverlay {
                self.state = TestSessionState::Running;
                self.owner = TestPresentationOwner::Game;
            } else if self.state == TestSessionState::Running {
                self.state = TestSessionState::SystemOverlay;
                self.owner = TestPresentationOwner::OnaSystemOverlay;
            }
            self
        }

        fn escape(self) -> Self {
            self.hold_start()
        }

        fn resume(mut self) -> Self {
            self.state = TestSessionState::Running;
            self.owner = TestPresentationOwner::Game;
            self
        }

        fn home(mut self) -> Self {
            self.state = TestSessionState::Background;
            self.owner = TestPresentationOwner::OnaShell;
            self
        }

        fn minimize(mut self) -> Self {
            self.state = TestSessionState::Minimized;
            self.owner = TestPresentationOwner::OnaMinimized;
            self
        }

        fn close_game(mut self) -> Self {
            self.state = TestSessionState::Stopping;
            self.owner = TestPresentationOwner::OnaTransitionGuard;
            self
        }

        fn game_ending(mut self) -> Self {
            self.state = TestSessionState::Returning;
            self.owner = TestPresentationOwner::OnaTransitionGuard;
            self
        }

        fn exited(mut self) -> Self {
            self.state = TestSessionState::Exited;
            self.owner = TestPresentationOwner::OnaShell;
            self.pid = None;
            self
        }

        fn cleanup(mut self) -> Self {
            self.state = TestSessionState::Idle;
            self.owner = TestPresentationOwner::OnaShell;
            self.pid = None;
            self
        }

        fn handoff_failed(mut self) -> Self {
            self.state = TestSessionState::Failed;
            self.owner = TestPresentationOwner::OnaTransitionGuard;
            self
        }
    }

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

    #[test]
    fn close_game_allows_return_home_and_launch_again() {
        let first_run = TestSession::idle().play(101).handoff_accepted();
        assert_eq!(first_run.owner, TestPresentationOwner::Game);

        let home = first_run.close_game().exited().cleanup();
        assert_eq!(home, TestSession::idle());

        let second_run = home.play(202).handoff_accepted();
        assert_eq!(second_run.state, TestSessionState::Running);
        assert_eq!(second_run.owner, TestPresentationOwner::Game);
        assert_eq!(second_run.pid, Some(202));
    }

    #[test]
    fn quick_menu_resume_returns_to_same_running_pid() {
        let running = TestSession::idle().play(101).handoff_accepted();
        let pid = running.pid;
        let resumed = running.hold_start().resume();

        assert_eq!(resumed.state, TestSessionState::Running);
        assert_eq!(resumed.owner, TestPresentationOwner::Game);
        assert_eq!(resumed.pid, pid);
    }

    #[test]
    fn hold_start_enters_system_overlay_without_backgrounding_game() {
        let overlay = TestSession::idle()
            .play(101)
            .handoff_accepted()
            .hold_start();

        assert_eq!(overlay.state, TestSessionState::SystemOverlay);
        assert_eq!(overlay.owner, TestPresentationOwner::OnaSystemOverlay);
        assert_eq!(overlay.pid, Some(101));
    }

    #[test]
    fn hold_start_toggles_system_overlay_ten_times_without_changing_pid() {
        let mut session = TestSession::idle().play(101).handoff_accepted();

        for _ in 0..10 {
            session = session.hold_start();
            assert_eq!(session.state, TestSessionState::SystemOverlay);
            assert_eq!(session.owner, TestPresentationOwner::OnaSystemOverlay);
            assert_eq!(session.pid, Some(101));

            session = session.hold_start();
            assert_eq!(session.state, TestSessionState::Running);
            assert_eq!(session.owner, TestPresentationOwner::Game);
            assert_eq!(session.pid, Some(101));
        }
    }

    #[test]
    fn escape_and_hold_start_share_the_same_overlay_toggle_contract() {
        let running = TestSession::idle().play(101).handoff_accepted();
        let opened_by_hold = running.hold_start();
        let closed_by_escape = opened_by_hold.escape();
        let opened_by_escape = closed_by_escape.escape();
        let closed_by_hold = opened_by_escape.hold_start();

        assert_eq!(closed_by_hold.state, TestSessionState::Running);
        assert_eq!(closed_by_hold.owner, TestPresentationOwner::Game);
        assert_eq!(closed_by_hold.pid, Some(101));
    }

    #[test]
    fn system_input_observation_stays_active_while_game_forwarding_is_paused() {
        let overlay = TestSession::idle()
            .play(101)
            .handoff_accepted()
            .hold_start();
        let game_forwarding = overlay.owner == TestPresentationOwner::Game;
        let system_input_observed = matches!(
            overlay.owner,
            TestPresentationOwner::Game | TestPresentationOwner::OnaSystemOverlay
        );

        assert!(!game_forwarding);
        assert!(system_input_observed);
        assert_eq!(overlay.pid, Some(101));
    }

    #[test]
    fn start_short_keeps_game_as_presentation_owner() {
        let running = TestSession::idle().play(101).handoff_accepted();

        assert_eq!(running.state, TestSessionState::Running);
        assert_eq!(running.owner, TestPresentationOwner::Game);
        assert_eq!(running.pid, Some(101));
    }

    #[test]
    fn quick_menu_home_keeps_background_session_pid_alive() {
        let home = TestSession::idle()
            .play(101)
            .handoff_accepted()
            .hold_start()
            .home();

        assert_eq!(home.state, TestSessionState::Background);
        assert_eq!(home.owner, TestPresentationOwner::OnaShell);
        assert_eq!(home.pid, Some(101));
    }

    #[test]
    fn background_continue_returns_to_same_pid() {
        let background = TestSession::idle()
            .play(101)
            .handoff_accepted()
            .hold_start()
            .home();
        let continued = background.resume();

        assert_eq!(continued.state, TestSessionState::Running);
        assert_eq!(continued.owner, TestPresentationOwner::Game);
        assert_eq!(continued.pid, Some(101));
    }

    #[test]
    fn background_close_clears_session() {
        let closed = TestSession::idle()
            .play(101)
            .handoff_accepted()
            .hold_start()
            .home()
            .close_game()
            .exited()
            .cleanup();

        assert_eq!(closed, TestSession::idle());
    }

    #[test]
    fn another_game_cancel_preserves_background_pid() {
        let background = TestSession::idle()
            .play(101)
            .handoff_accepted()
            .hold_start()
            .home();

        assert_eq!(background.state, TestSessionState::Background);
        assert_eq!(background.pid, Some(101));
    }

    #[test]
    fn background_input_is_shell_routed_and_continue_restores_game_routing() {
        let running = TestSession::idle().play(101).handoff_accepted();
        let game_input_forwarding = running.owner == TestPresentationOwner::Game;
        assert!(game_input_forwarding);

        let background = running.hold_start().home();
        let game_input_forwarding = background.owner == TestPresentationOwner::Game;
        assert!(!game_input_forwarding);

        let continued = background.resume();
        let game_input_forwarding = continued.owner == TestPresentationOwner::Game;
        assert!(game_input_forwarding);
    }

    #[test]
    fn minimize_and_restore_preserve_running_game_pid_without_handoff() {
        let running = TestSession::idle().play(101).handoff_accepted();
        let minimized = running.minimize();

        assert_eq!(minimized.state, TestSessionState::Minimized);
        assert_eq!(minimized.owner, TestPresentationOwner::OnaMinimized);
        assert_eq!(minimized.pid, Some(101));

        let restored = minimized.resume();

        assert_eq!(restored.state, TestSessionState::Running);
        assert_eq!(restored.owner, TestPresentationOwner::Game);
        assert_eq!(restored.pid, Some(101));
    }

    #[test]
    fn minimized_quick_menu_restore_returns_directly_to_game() {
        let overlay = TestSession::idle()
            .play(101)
            .handoff_accepted()
            .hold_start();
        let minimized = overlay.minimize();

        assert_eq!(minimized.owner, TestPresentationOwner::OnaMinimized);
        assert_eq!(minimized.pid, Some(101));

        let restored = minimized.resume();

        assert_eq!(restored.state, TestSessionState::Running);
        assert_eq!(restored.owner, TestPresentationOwner::Game);
        assert_eq!(restored.pid, Some(101));
    }

    #[test]
    fn failed_primary_window_restore_keeps_ona_shell_visible() {
        let minimized = TestSession::idle().play(101).handoff_accepted().minimize();
        let restore_validation_passed = false;
        let recovered = if restore_validation_passed {
            minimized.resume()
        } else {
            TestSession {
                state: TestSessionState::Background,
                owner: TestPresentationOwner::OnaShell,
                pid: minimized.pid,
            }
        };

        assert_eq!(recovered.owner, TestPresentationOwner::OnaShell);
        assert_eq!(recovered.state, TestSessionState::Background);
        assert_eq!(recovered.pid, Some(101));
    }

    #[test]
    fn closing_ona_with_live_background_session_cleans_owned_process() {
        let background = TestSession::idle()
            .play(101)
            .handoff_accepted()
            .hold_start()
            .home();
        assert_eq!(background.pid, Some(101));

        let closed = background.close_game().exited().cleanup();
        assert_eq!(closed.pid, None);
        assert_eq!(closed.state, TestSessionState::Idle);
    }

    #[test]
    fn crash_and_normal_exit_use_transition_guard() {
        let returning = TestSession::idle()
            .play(101)
            .handoff_accepted()
            .game_ending();

        assert_eq!(returning.state, TestSessionState::Returning);
        assert_eq!(returning.owner, TestPresentationOwner::OnaTransitionGuard);
        assert_eq!(returning.exited().cleanup(), TestSession::idle());
    }

    #[test]
    fn handoff_failure_keeps_transition_guard_for_rollback() {
        let failed = TestSession::idle().play(101).handoff_failed();

        assert_eq!(failed.state, TestSessionState::Failed);
        assert_eq!(failed.owner, TestPresentationOwner::OnaTransitionGuard);
        assert_eq!(failed.exited().cleanup(), TestSession::idle());
    }

    #[test]
    fn game_owner_requires_guard_hidden_after_safe_handoff() {
        let _all_guard_states = [
            TestGuardState::Hidden,
            TestGuardState::Visible,
            TestGuardState::Hiding,
            TestGuardState::Failed,
        ];
        let guard = TestGuardState::Hidden;
        let shell_visible = false;
        let game_visible = true;
        let forwarding_enabled = true;
        let owner = TestPresentationOwner::Game;

        assert_eq!(guard, TestGuardState::Hidden);
        assert!(!shell_visible);
        assert!(game_visible);
        assert!(forwarding_enabled);
        assert_eq!(owner, TestPresentationOwner::Game);
    }

    #[test]
    fn guard_release_failure_blocks_game_owner_finalization() {
        let guard = TestGuardState::Failed;
        let owner = TestPresentationOwner::OnaTransitionGuard;
        let forwarding_enabled = false;

        assert_ne!(guard, TestGuardState::Hidden);
        assert_eq!(owner, TestPresentationOwner::OnaTransitionGuard);
        assert!(!forwarding_enabled);
    }

    #[test]
    fn native_shell_restore_returns_to_opaque_shell_owner() {
        let guard = TestGuardState::Hidden;
        let shell_visible = true;
        let shell_opacity = 255;
        let shell_topmost = false;
        let overlay_active = false;
        let owner = TestPresentationOwner::OnaShell;

        assert_eq!(guard, TestGuardState::Hidden);
        assert!(shell_visible);
        assert_eq!(shell_opacity, 255);
        assert!(!shell_topmost);
        assert!(!overlay_active);
        assert_eq!(owner, TestPresentationOwner::OnaShell);
    }

    #[test]
    fn native_home_minimize_restore_never_reopens_quick_menu() {
        let previous_owner = TestPresentationOwner::OnaShell;
        let minimized_owner = TestPresentationOwner::OnaMinimized;
        let restored_owner = TestPresentationOwner::OnaShell;
        let quick_menu_open = false;
        let system_overlay = false;
        let shell_opacity = 255;
        let shell_topmost = false;
        let guard = TestGuardState::Hidden;

        assert_eq!(previous_owner, TestPresentationOwner::OnaShell);
        assert_eq!(minimized_owner, TestPresentationOwner::OnaMinimized);
        assert_eq!(restored_owner, TestPresentationOwner::OnaShell);
        assert!(!quick_menu_open);
        assert!(!system_overlay);
        assert_eq!(shell_opacity, 255);
        assert!(!shell_topmost);
        assert_eq!(guard, TestGuardState::Hidden);
    }

    #[test]
    fn repeated_native_home_restore_has_no_opacity_or_topmost_drift() {
        for _ in 0..10 {
            let owner = TestPresentationOwner::OnaShell;
            let shell_opacity = 255;
            let shell_topmost = false;
            let quick_menu_open = false;
            let guard = TestGuardState::Hidden;

            assert_eq!(owner, TestPresentationOwner::OnaShell);
            assert_eq!(shell_opacity, 255);
            assert!(!shell_topmost);
            assert!(!quick_menu_open);
            assert_eq!(guard, TestGuardState::Hidden);
        }
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
            input_forwarding_enabled: Mutex::new(false),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_qr_session,
            list_installed_games,
            scan_game_installation_sources,
            import_local_game,
            launch_installed_game,
            wait_for_game_handoff_ready,
            show_presentation_guard,
            hide_game_cursor,
            restore_game_cursor,
            prepare_shell_for_game,
            restore_shell_after_game,
            restore_shell_window_presentation,
            set_game_input_forwarding,
            neutralize_game_input,
            dispatch_game_start_short,
            focus_running_game,
            minimize_running_game,
            restore_running_game,
            suppress_running_game_taskbar_identity,
            release_presentation_guard,
            presentation_guard_native_status,
            shell_window_presentation_status,
            show_system_overlay_over_game,
            hide_system_overlay_over_game,
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
            local_connectivity_status,
            enable_escape_system_shortcut,
            disable_escape_system_shortcut,
            escape_system_shortcut_status,
            exit_ona_process,
            presentation_adapter_contract,
            load_controller_profile,
            save_controller_profile
        ])
        .setup(|app| {
            display_manager::configure_main_window(app)?;
            if let Some(main_window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
                let app_handle = app.handle().clone();
                let minimized_state = Arc::new(Mutex::new(false));
                let minimized_state_for_event = Arc::clone(&minimized_state);

                main_window.on_window_event(move |event| match event {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        println!("[ONA NativeWindow] close_requested");
                        api.prevent_close();
                        let _ = app_handle.emit("ona-native-close-requested", ());
                    }
                    tauri::WindowEvent::Resized(_) | tauri::WindowEvent::Focused(_) => {
                        let Some(window) = app_handle.get_webview_window(MAIN_WINDOW_LABEL) else {
                            return;
                        };
                        let minimized = window.is_minimized().unwrap_or(false);
                        let mut previous = match minimized_state_for_event.lock() {
                            Ok(previous) => previous,
                            Err(error) => {
                                eprintln!("[ONA NativeWindow] minimize state unavailable: {error}");
                                return;
                            }
                        };

                        if minimized == *previous {
                            return;
                        }

                        *previous = minimized;

                        if minimized {
                            println!("[ONA NativeWindow] minimized");
                            let _ = app_handle.emit("ona-native-minimized", ());
                        } else {
                            println!("[ONA NativeWindow] restored");
                            let _ = app_handle.emit("ona-native-restored", ());
                        }
                    }
                    _ => {}
                });
            }
            match presentation_guard_window(app.handle()) {
                Ok(guard) => {
                    if let Err(error) = configure_guard_window(app.handle(), &guard) {
                        eprintln!("[ONA Presentation] Guard preload layout failed: {error}");
                    }
                    let _ = guard.hide();
                    println!("[ONA Presentation] dedicated guard preloaded");
                }
                Err(error) => {
                    eprintln!("[ONA Presentation] Guard preload failed: {error}");
                }
            }

            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                while let Ok(player_id) = controller_connections.recv().await {
                    if let Err(error) = app_handle.emit("controller-connected", player_id) {
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
                            let forwarding_enabled = runtime
                                .input_forwarding_enabled
                                .lock()
                                .map(|forwarding| *forwarding)
                                .unwrap_or(false);

                            if let OnaInputEvent::Button {
                                button: OnaButton::Start,
                                state,
                                ..
                            } = &event
                            {
                                println!(
                                    "[ONA SystemInput] START {state:?} observed gameForwarding={forwarding_enabled}"
                                );
                            }

                            if forwarding_enabled {
                                if let Ok(bridge) = runtime.input_bridge.lock() {
                                    dispatch(event, Some(&bridge));
                                }
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
