use serde::Serialize;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tauri::{App, AppHandle, Manager, Monitor, PhysicalPosition, PhysicalSize, WebviewWindow, Wry};

const MAIN_WINDOW_LABEL: &str = "main";
const DISPLAY_RECOVERY_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayInfo {
    pub index: usize,
    pub identifier: String,
    pub name: Option<String>,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f64,
    pub is_primary: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayLayout {
    pub displays: Vec<DisplayInfo>,
    pub primary_index: Option<usize>,
    pub target_index: Option<usize>,
}

#[derive(Clone, Debug, PartialEq)]
struct DisplaySnapshot {
    identifier: String,
    is_primary: bool,
}

pub fn configure_main_window(app: &mut App<Wry>) -> tauri::Result<()> {
    let layout = detect_from_app(app)?;
    log_layout("initial", &layout);

    if let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        apply_layout_to_window(&window, &layout)?;
    } else {
        eprintln!("[ONA Display] Main window not found; display targeting was skipped.");
    }

    start_display_recovery(app.handle().clone(), layout.snapshot());

    Ok(())
}

fn detect_from_app(app: &App<Wry>) -> tauri::Result<DisplayLayout> {
    let monitors = app.available_monitors()?;
    let primary_monitor = app.primary_monitor()?;

    Ok(DisplayLayout::from_monitors(monitors, primary_monitor))
}

fn detect_from_handle(app_handle: &AppHandle<Wry>) -> tauri::Result<DisplayLayout> {
    let monitors = app_handle.available_monitors()?;
    let primary_monitor = app_handle.primary_monitor()?;

    Ok(DisplayLayout::from_monitors(monitors, primary_monitor))
}

pub fn detect_layout(app_handle: &AppHandle<Wry>) -> tauri::Result<DisplayLayout> {
    detect_from_handle(app_handle)
}

impl DisplayLayout {
    fn from_monitors(monitors: Vec<Monitor>, primary_monitor: Option<Monitor>) -> Self {
        let primary_signature = primary_monitor.as_ref().map(DisplaySignature::from_monitor);
        let primary_index = primary_signature
            .as_ref()
            .and_then(|signature| {
                monitors
                    .iter()
                    .position(|monitor| signature == &DisplaySignature::from_monitor(monitor))
            })
            .or_else(|| {
                monitors
                    .iter()
                    .position(|monitor| monitor.position().x == 0 && monitor.position().y == 0)
            })
            .or_else(|| (!monitors.is_empty()).then_some(0));

        let displays = monitors
            .iter()
            .enumerate()
            .map(|(index, monitor)| DisplayInfo::from_monitor(index, monitor, primary_index))
            .collect::<Vec<_>>();

        let target_index = choose_target_display(&displays, primary_index);

        Self {
            displays,
            primary_index,
            target_index,
        }
    }

    pub fn target_display(&self) -> Option<&DisplayInfo> {
        self.target_index.and_then(|target_index| {
            self.displays
                .iter()
                .find(|display| display.index == target_index)
        })
    }

    fn snapshot(&self) -> Vec<DisplaySnapshot> {
        self.displays
            .iter()
            .map(|display| DisplaySnapshot {
                identifier: display.identifier.clone(),
                is_primary: display.is_primary,
            })
            .collect()
    }
}

impl DisplayInfo {
    fn from_monitor(index: usize, monitor: &Monitor, primary_index: Option<usize>) -> Self {
        let position = monitor.position();
        let size = monitor.size();
        let name = monitor.name().cloned();
        let identifier = build_identifier(
            index,
            name.as_deref(),
            position.x,
            position.y,
            size.width,
            size.height,
        );

        Self {
            index,
            identifier,
            name,
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
            scale_factor: monitor.scale_factor(),
            is_primary: primary_index == Some(index),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DisplaySignature {
    name: Option<String>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl DisplaySignature {
    fn from_monitor(monitor: &Monitor) -> Self {
        let position = monitor.position();
        let size = monitor.size();

        Self {
            name: monitor.name().cloned(),
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        }
    }
}

fn choose_target_display(displays: &[DisplayInfo], primary_index: Option<usize>) -> Option<usize> {
    if displays.len() <= 1 {
        return primary_index.or_else(|| displays.first().map(|display| display.index));
    }

    displays
        .iter()
        .find(|display| !display.is_primary)
        .map(|display| display.index)
        .or_else(|| displays.get(1).map(|display| display.index))
        .or(primary_index)
}

pub fn apply_layout_to_window(
    window: &WebviewWindow<Wry>,
    layout: &DisplayLayout,
) -> tauri::Result<()> {
    let Some(target) = layout.target_display() else {
        eprintln!("[ONA Display] No monitors detected; keeping the current window placement.");
        return Ok(());
    };

    println!(
        "[ONA Display] Target display selected: #{} {} ({})",
        target.index,
        target.identifier,
        if target.is_primary {
            "primary"
        } else {
            "secondary"
        }
    );

    window.set_fullscreen(false)?;
    window.set_position(PhysicalPosition::new(target.x, target.y))?;
    window.set_size(PhysicalSize::new(target.width, target.height))?;
    window.set_fullscreen(true)?;

    Ok(())
}

fn start_display_recovery(app_handle: AppHandle<Wry>, initial_snapshot: Vec<DisplaySnapshot>) {
    let last_snapshot = Arc::new(Mutex::new(initial_snapshot));

    tauri::async_runtime::spawn_blocking(move || loop {
        thread::sleep(DISPLAY_RECOVERY_INTERVAL);

        let layout = match detect_from_handle(&app_handle) {
            Ok(layout) => layout,
            Err(error) => {
                eprintln!("[ONA Display] Could not refresh monitor list: {error}");
                continue;
            }
        };

        let snapshot = layout.snapshot();
        let changed = match last_snapshot.lock() {
            Ok(mut previous_snapshot) => {
                if *previous_snapshot == snapshot {
                    false
                } else {
                    *previous_snapshot = snapshot;
                    true
                }
            }
            Err(error) => {
                eprintln!("[ONA Display] Display recovery state is unavailable: {error}");
                false
            }
        };

        if !changed {
            continue;
        }

        log_layout("changed", &layout);

        if let Some(window) = app_handle.get_webview_window(MAIN_WINDOW_LABEL) {
            if let Err(error) = apply_layout_to_window(&window, &layout) {
                eprintln!("[ONA Display] Could not apply display target: {error}");
            }
        }
    });
}

fn build_identifier(
    index: usize,
    name: Option<&str>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> String {
    let monitor_name = name
        .filter(|name| !name.trim().is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| format!("display-{index}"));

    format!("{monitor_name}@{x},{y}:{width}x{height}")
}

fn log_layout(reason: &str, layout: &DisplayLayout) {
    println!(
        "[ONA Display] Monitor layout {reason}: {} display(s), primary={:?}, target={:?}",
        layout.displays.len(),
        layout.primary_index,
        layout.target_index
    );

    for display in &layout.displays {
        println!(
            "[ONA Display] Display #{}: {} | position=({}, {}) | resolution={}x{} | scale={} | role={}",
            display.index,
            display
                .name
                .as_deref()
                .filter(|name| !name.trim().is_empty())
                .unwrap_or(&display.identifier),
            display.x,
            display.y,
            display.width,
            display.height,
            display.scale_factor,
            if display.is_primary {
                "primary"
            } else {
                "secondary"
            }
        );
    }
}
