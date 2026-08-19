use std::{
    sync::{Mutex, OnceLock},
    thread,
    time::{Duration, Instant},
};

const POLL_INTERVAL: Duration = Duration::from_millis(80);
const MIN_PRIMARY_WINDOW_DIMENSION: i32 = 64;
static PRIMARY_GAME_WINDOW: OnceLock<Mutex<Option<StoredPrimaryGameWindow>>> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct TargetDisplayBounds {
    pub display_id: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub presentation_mode: String,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameHandoffStatus {
    pub process_ready: bool,
    pub window_ready: bool,
    pub window_on_target_display: bool,
    pub display_ready: bool,
    pub game_ready: bool,
    pub handshake_confirmed: bool,
    pub legacy_fallback: bool,
    pub ona_compatible: bool,
    pub presentation_valid: bool,
    pub rejection_reason: Option<String>,
    pub diagnostics: Option<GameHandoffDiagnostics>,
    pub foreground_attempted: bool,
    pub foreground_granted: bool,
    pub primary_window: Option<PrimaryGameWindowIdentity>,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryGameWindowIdentity {
    pub game_id: Option<String>,
    pub pid: u32,
    pub hwnd: String,
    pub window_class: String,
    pub window_title: String,
    pub expected_bounds: BoundsSnapshot,
    pub expected_monitor: String,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameWindowRestoreStatus {
    pub restored: bool,
    pub primary_hwnd: String,
    pub exists: bool,
    pub visible: bool,
    pub minimized: bool,
    pub rect: Option<BoundsSnapshot>,
    pub rect_compatible: bool,
    pub client_rect_compatible: bool,
    pub on_target_display: bool,
    pub foreground_granted: bool,
}

#[derive(Clone, Debug)]
struct StoredPrimaryGameWindow {
    game_id: Option<String>,
    pid: u32,
    raw_hwnd: isize,
    window_class: String,
    window_title: String,
    expected_bounds: BoundsSnapshot,
    expected_monitor: String,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameHandoffDiagnostics {
    pub expected_bounds: BoundsSnapshot,
    pub detected_bounds: Option<BoundsSnapshot>,
    pub detected_client_bounds: Option<BoundsSnapshot>,
    pub expected_monitor: String,
    pub detected_monitor: Option<String>,
    pub window_style: Option<WindowStyleSnapshot>,
    pub game_ready_received: bool,
    pub ona_compatible: bool,
    pub presentation_valid: bool,
    pub rejection_reason: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundsSnapshot {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowStyleSnapshot {
    pub style_hex: String,
    pub ex_style_hex: String,
    pub has_caption: bool,
    pub has_border: bool,
    pub has_dialog_frame: bool,
    pub has_thick_frame: bool,
}

pub fn wait_for_game_handoff_ready(
    pid: u32,
    game_id: Option<String>,
    target: TargetDisplayBounds,
    timeout_ms: u64,
    mut has_ona_lifecycle: impl FnMut() -> bool,
    mut handshake_display_ready: impl FnMut() -> bool,
    mut handshake_game_ready: impl FnMut() -> bool,
) -> GameHandoffStatus {
    let timeout = Duration::from_millis(timeout_ms.max(1));
    let started = Instant::now();
    let mut window_ready = false;
    let mut window_on_target_display = false;
    let mut handshake_confirmed = false;
    let mut game_ready = false;
    let mut ona_compatible = false;
    let mut last_presentation_valid = false;
    let mut foreground_attempted = false;
    let mut foreground_granted = false;
    let mut last_valid_window = None;
    let mut last_diagnostics = handoff_diagnostics(&target, None, game_ready, ona_compatible);

    while started.elapsed() < timeout {
        if has_ona_lifecycle() {
            ona_compatible = true;
        }

        if handshake_display_ready() {
            ona_compatible = true;
            handshake_confirmed = true;
        }

        if handshake_game_ready() {
            ona_compatible = true;
            handshake_confirmed = true;
            game_ready = true;
        }

        if let Some(window) = platform_best_window_for_pid(pid, Some(&target), None) {
            window_ready = true;
            window_on_target_display = window_is_on_target_display(&window, &target);
            let presentation = validate_window_presentation(&window, &target);
            last_presentation_valid = presentation.valid;
            if presentation.valid {
                last_valid_window = Some(window.clone());
            }
            last_diagnostics =
                handoff_diagnostics(&target, Some(&presentation), game_ready, ona_compatible);

            if window_on_target_display && !foreground_attempted {
                foreground_attempted = true;
                foreground_granted = platform_try_foreground_window(&window);
            }

            if game_ready && presentation.valid {
                let primary_window =
                    capture_primary_game_window(pid, game_id.clone(), &window, &target);
                return GameHandoffStatus {
                    process_ready: true,
                    window_ready,
                    window_on_target_display,
                    display_ready: true,
                    game_ready,
                    handshake_confirmed,
                    legacy_fallback: false,
                    ona_compatible,
                    presentation_valid: true,
                    rejection_reason: None,
                    diagnostics: Some(last_diagnostics),
                    foreground_attempted,
                    foreground_granted,
                    primary_window: Some(primary_window),
                };
            }
        }

        thread::sleep(POLL_INTERVAL);
    }

    if should_return_legacy_fallback(last_presentation_valid, ona_compatible) {
        let primary_window = last_valid_window
            .as_ref()
            .map(|window| capture_primary_game_window(pid, game_id, window, &target));
        return GameHandoffStatus {
            process_ready: true,
            window_ready,
            window_on_target_display,
            display_ready: false,
            game_ready: false,
            handshake_confirmed: false,
            legacy_fallback: true,
            ona_compatible: false,
            presentation_valid: true,
            rejection_reason: None,
            diagnostics: Some(last_diagnostics),
            foreground_attempted,
            foreground_granted,
            primary_window,
        };
    }

    if ona_compatible && !game_ready {
        last_diagnostics.rejection_reason =
            Some("ONA_COMPATIBLE_GAME_READY_NOT_RECEIVED".to_string());
        last_diagnostics.presentation_valid = last_presentation_valid;
    }

    GameHandoffStatus {
        process_ready: true,
        window_ready,
        window_on_target_display,
        display_ready: false,
        game_ready,
        handshake_confirmed,
        legacy_fallback: false,
        ona_compatible,
        presentation_valid: last_presentation_valid,
        rejection_reason: if ona_compatible && !game_ready {
            Some("ONA_COMPATIBLE_GAME_READY_NOT_RECEIVED".to_string())
        } else {
            last_diagnostics.rejection_reason.clone()
        },
        diagnostics: Some(last_diagnostics),
        foreground_attempted,
        foreground_granted,
        primary_window: None,
    }
}

pub fn clear_primary_game_window() {
    if let Ok(mut primary) = primary_game_window_state().lock() {
        *primary = None;
    }
}

pub fn focus_process_window(pid: u32) -> Result<bool, String> {
    let (primary, window) = primary_window_for_operation(pid)?;
    println!(
        "[ONA GameWindow] focus using PRIMARY hwnd={} pid={pid}",
        format_hwnd(primary.raw_hwnd)
    );
    Ok(platform_try_foreground_window(&window))
}

pub fn minimize_process_window(pid: u32) -> Result<bool, String> {
    let (primary, window) = primary_window_for_operation(pid)?;
    println!(
        "[ONA GameWindow] minimize using PRIMARY hwnd={} pid={pid}",
        format_hwnd(primary.raw_hwnd)
    );
    Ok(platform_minimize_window(&window))
}

pub fn restore_process_window(pid: u32) -> Result<GameWindowRestoreStatus, String> {
    let (primary, window) = primary_window_for_operation(pid)?;
    println!(
        "[ONA GameWindow] restore using PRIMARY hwnd={} pid={pid}",
        format_hwnd(primary.raw_hwnd)
    );
    Ok(platform_restore_window(&window, &primary))
}

pub fn suppress_process_taskbar_identity(pid: u32) -> Result<bool, String> {
    let (_, window) = primary_window_for_operation(pid)?;
    platform_suppress_taskbar_identity(&window)
}

fn primary_game_window_state() -> &'static Mutex<Option<StoredPrimaryGameWindow>> {
    PRIMARY_GAME_WINDOW.get_or_init(|| Mutex::new(None))
}

fn format_hwnd(raw_hwnd: isize) -> String {
    format!("0x{:x}", raw_hwnd as usize)
}

impl StoredPrimaryGameWindow {
    fn identity(&self) -> PrimaryGameWindowIdentity {
        PrimaryGameWindowIdentity {
            game_id: self.game_id.clone(),
            pid: self.pid,
            hwnd: format_hwnd(self.raw_hwnd),
            window_class: self.window_class.clone(),
            window_title: self.window_title.clone(),
            expected_bounds: self.expected_bounds.clone(),
            expected_monitor: self.expected_monitor.clone(),
        }
    }

    fn target(&self) -> TargetDisplayBounds {
        TargetDisplayBounds {
            display_id: self.expected_monitor.clone(),
            x: self.expected_bounds.x,
            y: self.expected_bounds.y,
            width: self.expected_bounds.width.max(0) as u32,
            height: self.expected_bounds.height.max(0) as u32,
            presentation_mode: "CONSOLE_FULLSCREEN".to_string(),
        }
    }
}

fn capture_primary_game_window(
    pid: u32,
    game_id: Option<String>,
    window: &WindowBounds,
    target: &TargetDisplayBounds,
) -> PrimaryGameWindowIdentity {
    let stored = StoredPrimaryGameWindow {
        game_id,
        pid,
        raw_hwnd: window.raw_hwnd,
        window_class: window.window_class.clone(),
        window_title: window.window_title.clone(),
        expected_bounds: BoundsSnapshot {
            x: target.x,
            y: target.y,
            width: target.width as i32,
            height: target.height as i32,
        },
        expected_monitor: target.display_id.clone(),
    };
    let identity = stored.identity();
    let mut primary = primary_game_window_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *primary = Some(stored);
    println!(
        "[ONA GameWindow] PRIMARY HWND captured hwnd={} pid={pid}",
        identity.hwnd
    );
    identity
}

fn primary_window_for_operation(
    pid: u32,
) -> Result<(StoredPrimaryGameWindow, WindowBounds), String> {
    let stored = primary_game_window_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone()
        .ok_or_else(|| "PRIMARY_GAME_HWND_NOT_CAPTURED".to_string())?;

    if stored.pid != pid {
        return Err(format!(
            "PRIMARY_GAME_HWND_PID_MISMATCH: expected={} requested={pid}",
            stored.pid
        ));
    }

    if let Some(window) = platform_window_snapshot(stored.raw_hwnd) {
        let class_matches = stored.window_class.is_empty()
            || window
                .window_class
                .eq_ignore_ascii_case(&stored.window_class);

        if window.pid == pid && class_matches && !is_helper_window_class(&window.window_class) {
            return Ok((stored, window));
        }
    }

    let target = stored.target();
    let reacquired = platform_best_window_for_pid(pid, Some(&target), Some(&stored))
        .filter(|window| validate_window_presentation(window, &target).valid)
        .ok_or_else(|| "PRIMARY_GAME_HWND_REACQUIRE_FAILED".to_string())?;
    let mut updated = stored;
    updated.raw_hwnd = reacquired.raw_hwnd;
    updated.window_class = reacquired.window_class.clone();
    updated.window_title = reacquired.window_title.clone();

    println!(
        "[ONA GameWindow] PRIMARY HWND reacquired hwnd={} pid={pid}",
        format_hwnd(updated.raw_hwnd)
    );
    *primary_game_window_state()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(updated.clone());

    Ok((updated, reacquired))
}

#[cfg(windows)]
pub fn log_visible_windows(reason: &str) {
    use windows::Win32::{
        Foundation::{BOOL, HWND, LPARAM, RECT},
        UI::WindowsAndMessaging::{
            EnumWindows, GetClassNameW, GetClientRect, GetWindow, GetWindowLongPtrW, GetWindowRect,
            GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible, GWL_EXSTYLE, GWL_STYLE,
            GW_OWNER,
        },
    };

    unsafe extern "system" fn enum_window(window: HWND, lparam: LPARAM) -> BOOL {
        let reason = &*(lparam.0 as *const String);

        if !IsWindowVisible(window).as_bool() {
            return BOOL(1);
        }

        let mut pid = 0;
        GetWindowThreadProcessId(window, Some(&mut pid));

        let owner = GetWindow(window, GW_OWNER).ok();
        let mut owner_pid = 0;
        if let Some(owner_window) = owner {
            if owner_window.0 != std::ptr::null_mut() {
                GetWindowThreadProcessId(owner_window, Some(&mut owner_pid));
            }
        }

        let mut rect = RECT::default();
        let rect_text = if GetWindowRect(window, &mut rect).is_ok() {
            format!(
                "{},{},{}x{}",
                rect.left,
                rect.top,
                rect.right - rect.left,
                rect.bottom - rect.top
            )
        } else {
            "unavailable".to_string()
        };

        let client_text = if GetClientRect(window, &mut rect).is_ok() {
            format!(
                "{},{},{}x{}",
                rect.left,
                rect.top,
                rect.right - rect.left,
                rect.bottom - rect.top
            )
        } else {
            "unavailable".to_string()
        };

        let style = GetWindowLongPtrW(window, GWL_STYLE) as u32;
        let ex_style = GetWindowLongPtrW(window, GWL_EXSTYLE) as u32;

        let mut class_name = [0u16; 256];
        let class_len = GetClassNameW(window, &mut class_name);
        let class_name = String::from_utf16_lossy(&class_name[..class_len as usize]);

        let mut title = [0u16; 256];
        let title_len = GetWindowTextW(window, &mut title);
        let title = String::from_utf16_lossy(&title[..title_len as usize]);

        println!(
            "[ONA WindowDiag] reason={} hwnd={:?} pid={} ownerPid={} class=\"{}\" title=\"{}\" rect={} client={} visible=true style=0x{:08X} ex_style=0x{:08X}",
            reason,
            window.0,
            pid,
            owner_pid,
            class_name,
            title,
            rect_text,
            client_text,
            style,
            ex_style
        );

        BOOL(1)
    }

    let reason = reason.to_string();

    unsafe {
        let _ = EnumWindows(Some(enum_window), LPARAM(&reason as *const String as isize));
    }
}

#[cfg(not(windows))]
pub fn log_visible_windows(_reason: &str) {}

fn should_return_legacy_fallback(presentation_valid: bool, ona_compatible: bool) -> bool {
    presentation_valid && !ona_compatible
}

fn window_is_on_target_display(window: &WindowBounds, target: &TargetDisplayBounds) -> bool {
    if let Some(window_display_id) = &window.display_id {
        return window_display_id.eq_ignore_ascii_case(&target.display_id);
    }

    let target_right = target.x + target.width as i32;
    let target_bottom = target.y + target.height as i32;
    let window_right = window.x + window.width;
    let window_bottom = window.y + window.height;

    window.x < target_right
        && window_right > target.x
        && window.y < target_bottom
        && window_bottom > target.y
}

fn validate_window_presentation(
    window: &WindowBounds,
    target: &TargetDisplayBounds,
) -> WindowPresentationValidation {
    let expected = BoundsSnapshot {
        x: target.x,
        y: target.y,
        width: target.width as i32,
        height: target.height as i32,
    };
    let detected = BoundsSnapshot {
        x: window.x,
        y: window.y,
        width: window.width,
        height: window.height,
    };
    let on_target_display = window_is_on_target_display(window, target);
    let bounds_match = bounds_cover_target(&detected, &expected);
    let client_bounds_match = window
        .client_bounds
        .as_ref()
        .map(|client| bounds_cover_target(client, &expected))
        .unwrap_or(false);
    let has_normal_window_chrome = window
        .style
        .as_ref()
        .map(|style| style.has_caption || style.has_dialog_frame || style.has_thick_frame)
        .unwrap_or(false);
    let requires_console_fullscreen = target
        .presentation_mode
        .eq_ignore_ascii_case("CONSOLE_FULLSCREEN");

    let rejection_reason = if !requires_console_fullscreen {
        Some("DISPLAY_MODE_IS_NOT_CONSOLE_FULLSCREEN".to_string())
    } else if !on_target_display {
        Some("WINDOW_NOT_ON_TARGET_DISPLAY".to_string())
    } else if !bounds_match {
        Some("WINDOW_BOUNDS_DO_NOT_MATCH_GAMING_DISPLAY".to_string())
    } else if !client_bounds_match {
        Some("WINDOW_CLIENT_BOUNDS_DO_NOT_MATCH_GAMING_DISPLAY".to_string())
    } else if has_normal_window_chrome {
        Some("WINDOW_STYLE_HAS_CAPTION_OR_FRAME".to_string())
    } else {
        None
    };

    WindowPresentationValidation {
        valid: rejection_reason.is_none(),
        rejection_reason,
        expected_bounds: expected,
        detected_bounds: Some(detected),
        detected_client_bounds: window.client_bounds.clone(),
        expected_monitor: target.display_id.clone(),
        detected_monitor: window.display_id.clone(),
        window_style: window.style.clone(),
    }
}

fn handoff_diagnostics(
    target: &TargetDisplayBounds,
    validation: Option<&WindowPresentationValidation>,
    game_ready: bool,
    ona_compatible: bool,
) -> GameHandoffDiagnostics {
    let expected_bounds = BoundsSnapshot {
        x: target.x,
        y: target.y,
        width: target.width as i32,
        height: target.height as i32,
    };

    if let Some(validation) = validation {
        return GameHandoffDiagnostics {
            expected_bounds: validation.expected_bounds.clone(),
            detected_bounds: validation.detected_bounds.clone(),
            detected_client_bounds: validation.detected_client_bounds.clone(),
            expected_monitor: validation.expected_monitor.clone(),
            detected_monitor: validation.detected_monitor.clone(),
            window_style: validation.window_style.clone(),
            game_ready_received: game_ready,
            ona_compatible,
            presentation_valid: validation.valid,
            rejection_reason: validation.rejection_reason.clone(),
        };
    }

    GameHandoffDiagnostics {
        expected_bounds,
        detected_bounds: None,
        detected_client_bounds: None,
        expected_monitor: target.display_id.clone(),
        detected_monitor: None,
        window_style: None,
        game_ready_received: game_ready,
        ona_compatible,
        presentation_valid: false,
        rejection_reason: Some("GAME_WINDOW_NOT_FOUND".to_string()),
    }
}

fn bounds_cover_target(detected: &BoundsSnapshot, expected: &BoundsSnapshot) -> bool {
    let tolerance = bounds_tolerance(expected);
    let detected_right = detected.x + detected.width;
    let detected_bottom = detected.y + detected.height;
    let expected_right = expected.x + expected.width;
    let expected_bottom = expected.y + expected.height;

    (detected.x - expected.x).abs() <= tolerance
        && (detected.y - expected.y).abs() <= tolerance
        && (detected_right - expected_right).abs() <= tolerance
        && (detected_bottom - expected_bottom).abs() <= tolerance
        && (detected.width - expected.width).abs() <= tolerance
        && (detected.height - expected.height).abs() <= tolerance
}

fn bounds_tolerance(expected: &BoundsSnapshot) -> i32 {
    ((expected.width.max(expected.height) as f32) * 0.01)
        .round()
        .max(8.0) as i32
}

fn restore_commit_allowed(
    exists: bool,
    visible: bool,
    minimized: bool,
    rect_compatible: bool,
    client_rect_compatible: bool,
    on_target_display: bool,
) -> bool {
    exists
        && visible
        && !minimized
        && rect_compatible
        && client_rect_compatible
        && on_target_display
}

struct WindowPresentationValidation {
    valid: bool,
    rejection_reason: Option<String>,
    expected_bounds: BoundsSnapshot,
    detected_bounds: Option<BoundsSnapshot>,
    detected_client_bounds: Option<BoundsSnapshot>,
    expected_monitor: String,
    detected_monitor: Option<String>,
    window_style: Option<WindowStyleSnapshot>,
}

#[derive(Clone, Debug)]
struct WindowBounds {
    raw_hwnd: isize,
    pid: u32,
    visible: bool,
    minimized: bool,
    is_tool_window: bool,
    window_class: String,
    window_title: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    client_bounds: Option<BoundsSnapshot>,
    display_id: Option<String>,
    style: Option<WindowStyleSnapshot>,
}

fn is_helper_window_class(window_class: &str) -> bool {
    let normalized = window_class.trim().to_ascii_lowercase();
    normalized == "winit thread event target"
        || normalized.contains("event target")
        || normalized.contains("message-only")
}

fn primary_candidate_rejection_reason(window: &WindowBounds) -> Option<&'static str> {
    if !window.visible {
        return Some("INVISIBLE");
    }
    if is_helper_window_class(&window.window_class) {
        return Some("HELPER_CLASS");
    }
    if window.is_tool_window {
        return Some("TOOL_WINDOW");
    }
    if window.width < MIN_PRIMARY_WINDOW_DIMENSION || window.height < MIN_PRIMARY_WINDOW_DIMENSION {
        return Some("WINDOW_TOO_SMALL");
    }
    None
}

fn window_candidate_score(
    window: &WindowBounds,
    target: Option<&TargetDisplayBounds>,
    preferred: Option<&StoredPrimaryGameWindow>,
) -> i64 {
    let mut score = 0i64;

    if let Some(target) = target {
        let validation = validate_window_presentation(window, target);
        if validation.valid {
            score += 1_000_000;
        }
        if window_is_on_target_display(window, target) {
            score += 100_000;
        }
        if validation
            .detected_bounds
            .as_ref()
            .is_some_and(|bounds| bounds_cover_target(bounds, &validation.expected_bounds))
        {
            score += 50_000;
        }
        if validation
            .detected_client_bounds
            .as_ref()
            .is_some_and(|bounds| bounds_cover_target(bounds, &validation.expected_bounds))
        {
            score += 50_000;
        }
    }

    if let Some(preferred) = preferred {
        if window
            .window_class
            .eq_ignore_ascii_case(&preferred.window_class)
        {
            score += 20_000;
        }
        if !preferred.window_title.is_empty()
            && window
                .window_title
                .eq_ignore_ascii_case(&preferred.window_title)
        {
            score += 10_000;
        }
    }

    if !window.window_title.trim().is_empty() {
        score += 1_000;
    }

    score + i64::from(window.width.max(0)) * i64::from(window.height.max(0)) / 1_000
}

fn platform_best_window_for_pid(
    pid: u32,
    target: Option<&TargetDisplayBounds>,
    preferred: Option<&StoredPrimaryGameWindow>,
) -> Option<WindowBounds> {
    platform_windows_for_pid(pid)
        .into_iter()
        .filter_map(|window| {
            if let Some(reason) = primary_candidate_rejection_reason(&window) {
                if matches!(reason, "HELPER_CLASS" | "TOOL_WINDOW" | "WINDOW_TOO_SMALL") {
                    println!(
                        "[ONA GameWindow] rejected helper hwnd={} class=\"{}\" reason={reason}",
                        format_hwnd(window.raw_hwnd),
                        window.window_class
                    );
                }
                return None;
            }

            let score = window_candidate_score(&window, target, preferred);
            Some((score, window))
        })
        .max_by_key(|(score, _)| *score)
        .map(|(_, window)| window)
}

#[cfg(windows)]
fn platform_windows_for_pid(pid: u32) -> Vec<WindowBounds> {
    use windows::Win32::{
        Foundation::{BOOL, HWND, LPARAM},
        UI::WindowsAndMessaging::{EnumWindows, GetWindowThreadProcessId},
    };

    struct SearchState {
        pid: u32,
        windows: Vec<WindowBounds>,
    }

    unsafe extern "system" fn enum_window(window: HWND, lparam: LPARAM) -> BOOL {
        let state = &mut *(lparam.0 as *mut SearchState);
        let mut window_pid = 0;

        GetWindowThreadProcessId(window, Some(&mut window_pid));

        if window_pid == state.pid {
            if let Some(snapshot) = platform_window_snapshot(window.0 as isize) {
                state.windows.push(snapshot);
            }
        }

        BOOL(1)
    }

    let mut state = SearchState {
        pid,
        windows: Vec::new(),
    };

    unsafe {
        let _ = EnumWindows(Some(enum_window), LPARAM(&mut state as *mut _ as isize));
    }

    state.windows
}

#[cfg(windows)]
fn platform_window_snapshot(raw_hwnd: isize) -> Option<WindowBounds> {
    use windows::Win32::{
        Foundation::{HWND, RECT},
        UI::WindowsAndMessaging::{
            GetClassNameW, GetWindow, GetWindowLongPtrW, GetWindowRect, GetWindowTextW,
            GetWindowThreadProcessId, IsIconic, IsWindow, IsWindowVisible, GWL_EXSTYLE, GW_OWNER,
            WS_EX_TOOLWINDOW,
        },
    };

    let window = HWND(raw_hwnd as *mut std::ffi::c_void);
    if !unsafe { IsWindow(window) }.as_bool() {
        return None;
    }

    let owner = unsafe { GetWindow(window, GW_OWNER) }.ok();
    if owner.is_some_and(|owner| owner.0 != std::ptr::null_mut()) {
        return None;
    }

    let mut pid = 0;
    unsafe { GetWindowThreadProcessId(window, Some(&mut pid)) };

    let mut rect = RECT::default();
    if unsafe { GetWindowRect(window, &mut rect) }.is_err() {
        return None;
    }

    let mut class_name = [0u16; 256];
    let class_len = unsafe { GetClassNameW(window, &mut class_name) };
    let window_class = String::from_utf16_lossy(&class_name[..class_len as usize]);
    let mut title = [0u16; 256];
    let title_len = unsafe { GetWindowTextW(window, &mut title) };
    let window_title = String::from_utf16_lossy(&title[..title_len as usize]);
    let ex_style = unsafe { GetWindowLongPtrW(window, GWL_EXSTYLE) } as u32;

    Some(WindowBounds {
        raw_hwnd,
        pid,
        visible: unsafe { IsWindowVisible(window) }.as_bool(),
        minimized: unsafe { IsIconic(window) }.as_bool(),
        is_tool_window: ex_style & WS_EX_TOOLWINDOW.0 != 0,
        window_class,
        window_title,
        x: rect.left,
        y: rect.top,
        width: rect.right - rect.left,
        height: rect.bottom - rect.top,
        client_bounds: client_bounds_for_window(window),
        display_id: display_id_for_window(window),
        style: style_for_window(window),
    })
}

#[cfg(windows)]
fn client_bounds_for_window(window: windows::Win32::Foundation::HWND) -> Option<BoundsSnapshot> {
    use windows::Win32::{
        Foundation::{POINT, RECT},
        Graphics::Gdi::ClientToScreen,
        UI::WindowsAndMessaging::GetClientRect,
    };

    let mut client = RECT::default();

    if unsafe { GetClientRect(window, &mut client) }.is_err() {
        return None;
    }

    let mut top_left = POINT {
        x: client.left,
        y: client.top,
    };
    let mut bottom_right = POINT {
        x: client.right,
        y: client.bottom,
    };

    if !unsafe { ClientToScreen(window, &mut top_left) }.as_bool() {
        return None;
    }

    if !unsafe { ClientToScreen(window, &mut bottom_right) }.as_bool() {
        return None;
    }

    Some(BoundsSnapshot {
        x: top_left.x,
        y: top_left.y,
        width: bottom_right.x - top_left.x,
        height: bottom_right.y - top_left.y,
    })
}

#[cfg(windows)]
fn style_for_window(window: windows::Win32::Foundation::HWND) -> Option<WindowStyleSnapshot> {
    use windows::Win32::UI::WindowsAndMessaging::{
        GetWindowLongPtrW, GWL_EXSTYLE, GWL_STYLE, WS_BORDER, WS_CAPTION, WS_DLGFRAME,
        WS_THICKFRAME,
    };

    let style = unsafe { GetWindowLongPtrW(window, GWL_STYLE) } as u32;
    let ex_style = unsafe { GetWindowLongPtrW(window, GWL_EXSTYLE) } as u32;

    Some(WindowStyleSnapshot {
        style_hex: format!("0x{style:08X}"),
        ex_style_hex: format!("0x{ex_style:08X}"),
        has_caption: style & WS_CAPTION.0 != 0,
        has_border: style & WS_BORDER.0 != 0,
        has_dialog_frame: style & WS_DLGFRAME.0 != 0,
        has_thick_frame: style & WS_THICKFRAME.0 != 0,
    })
}

#[cfg(windows)]
fn platform_try_foreground_window(window: &WindowBounds) -> bool {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{SetForegroundWindow, ShowWindow, SW_RESTORE};

    let hwnd = HWND(window.raw_hwnd as *mut std::ffi::c_void);
    unsafe {
        let _ = ShowWindow(hwnd, SW_RESTORE);
        SetForegroundWindow(hwnd).as_bool()
    }
}

#[cfg(not(windows))]
fn platform_try_foreground_window(_window: &WindowBounds) -> bool {
    false
}

#[cfg(windows)]
fn platform_minimize_window(window: &WindowBounds) -> bool {
    use windows::Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{IsIconic, ShowWindow, SW_MINIMIZE},
    };

    let hwnd = HWND(window.raw_hwnd as *mut std::ffi::c_void);
    unsafe {
        let _ = ShowWindow(hwnd, SW_MINIMIZE);
        IsIconic(hwnd).as_bool()
    }
}

#[cfg(not(windows))]
fn platform_minimize_window(_window: &WindowBounds) -> bool {
    false
}

#[cfg(windows)]
fn platform_restore_window(
    window: &WindowBounds,
    primary: &StoredPrimaryGameWindow,
) -> GameWindowRestoreStatus {
    use windows::Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{
            IsWindow, SetForegroundWindow, SetWindowPos, ShowWindow, SWP_NOZORDER, SWP_SHOWWINDOW,
            SW_RESTORE,
        },
    };

    let hwnd = HWND(window.raw_hwnd as *mut std::ffi::c_void);
    unsafe {
        let _ = ShowWindow(hwnd, SW_RESTORE);
        let _ = SetWindowPos(
            hwnd,
            None,
            primary.expected_bounds.x,
            primary.expected_bounds.y,
            primary.expected_bounds.width,
            primary.expected_bounds.height,
            SWP_NOZORDER | SWP_SHOWWINDOW,
        );
    }
    let foreground_granted = unsafe { SetForegroundWindow(hwnd) }.as_bool();
    let exists = unsafe { IsWindow(hwnd) }.as_bool();
    let restored_window = platform_window_snapshot(primary.raw_hwnd);
    let rect = restored_window.as_ref().map(|window| BoundsSnapshot {
        x: window.x,
        y: window.y,
        width: window.width,
        height: window.height,
    });
    let visible = restored_window
        .as_ref()
        .map(|window| window.visible)
        .unwrap_or(false);
    let minimized = restored_window
        .as_ref()
        .map(|window| window.minimized)
        .unwrap_or(true);
    let rect_compatible = rect
        .as_ref()
        .is_some_and(|bounds| bounds_cover_target(bounds, &primary.expected_bounds));
    let client_rect_compatible = restored_window
        .as_ref()
        .and_then(|window| window.client_bounds.as_ref())
        .is_some_and(|bounds| bounds_cover_target(bounds, &primary.expected_bounds));
    let target = primary.target();
    let on_target_display = restored_window
        .as_ref()
        .map(|window| window_is_on_target_display(window, &target))
        .unwrap_or(false);
    let restored = restore_commit_allowed(
        exists,
        visible,
        minimized,
        rect_compatible,
        client_rect_compatible,
        on_target_display,
    );
    let rect_log = rect
        .as_ref()
        .map(|bounds| {
            format!(
                "{},{},{}x{}",
                bounds.x, bounds.y, bounds.width, bounds.height
            )
        })
        .unwrap_or_else(|| "unavailable".to_string());

    println!(
        "[ONA GameWindow] restore validation visible={visible} minimized={minimized} rect={rect_log} compatible={rect_compatible} clientCompatible={client_rect_compatible} target={on_target_display}"
    );
    if restored {
        println!(
            "[ONA GameWindow] restore commit hwnd={}",
            format_hwnd(primary.raw_hwnd)
        );
    }

    GameWindowRestoreStatus {
        restored,
        primary_hwnd: format_hwnd(primary.raw_hwnd),
        exists,
        visible,
        minimized,
        rect,
        rect_compatible,
        client_rect_compatible,
        on_target_display,
        foreground_granted,
    }
}

#[cfg(not(windows))]
fn platform_restore_window(
    _window: &WindowBounds,
    primary: &StoredPrimaryGameWindow,
) -> GameWindowRestoreStatus {
    GameWindowRestoreStatus {
        restored: false,
        primary_hwnd: format_hwnd(primary.raw_hwnd),
        exists: false,
        visible: false,
        minimized: false,
        rect: None,
        rect_compatible: false,
        client_rect_compatible: false,
        on_target_display: false,
        foreground_granted: false,
    }
}

#[cfg(windows)]
fn platform_suppress_taskbar_identity(window: &WindowBounds) -> Result<bool, String> {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetWindowLongPtrW, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, SWP_FRAMECHANGED,
        SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW,
    };

    let hwnd = HWND(window.raw_hwnd as *mut std::ffi::c_void);
    let original_style = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) };
    let new_style =
        (original_style & !(WS_EX_APPWINDOW.0 as isize)) | (WS_EX_TOOLWINDOW.0 as isize);

    if new_style == original_style {
        println!(
            "[ONA GameWindow] taskbar already suppressed hwnd={} exStyle=0x{original_style:X}",
            format_hwnd(window.raw_hwnd)
        );
        return Ok(true);
    }

    let previous = unsafe { SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style) };

    if previous == 0 {
        let last_error = windows::core::Error::from_win32();
        return Err(format!("SET_WINDOW_EXSTYLE_FAILED: {last_error}"));
    }

    let _ = unsafe {
        SetWindowPos(
            hwnd,
            None,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
        )
    };

    println!(
        "[ONA GameWindow] taskbar suppressed hwnd={} originalExStyle=0x{original_style:X} newExStyle=0x{new_style:X}",
        format_hwnd(window.raw_hwnd)
    );

    Ok(true)
}

#[cfg(not(windows))]
fn platform_suppress_taskbar_identity(_window: &WindowBounds) -> Result<bool, String> {
    Ok(false)
}

#[cfg(windows)]
fn display_id_for_window(window: windows::Win32::Foundation::HWND) -> Option<String> {
    use windows::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromWindow, MONITORINFOEXW, MONITOR_DEFAULTTONEAREST,
    };

    let monitor = unsafe { MonitorFromWindow(window, MONITOR_DEFAULTTONEAREST) };
    let mut info = MONITORINFOEXW::default();
    info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

    if unsafe { GetMonitorInfoW(monitor, &mut info as *mut _ as *mut _) }.as_bool() {
        let name = String::from_utf16_lossy(&info.szDevice);
        return Some(name.trim_end_matches('\0').to_string())
            .filter(|display_id| !display_id.is_empty());
    }

    None
}

#[cfg(not(windows))]
fn platform_windows_for_pid(_pid: u32) -> Vec<WindowBounds> {
    Vec::new()
}

#[cfg(not(windows))]
fn platform_window_snapshot(_raw_hwnd: isize) -> Option<WindowBounds> {
    None
}

#[cfg(test)]
mod tests {
    use super::{
        bounds_cover_target, primary_candidate_rejection_reason, restore_commit_allowed,
        should_return_legacy_fallback, window_candidate_score, BoundsSnapshot,
        StoredPrimaryGameWindow, TargetDisplayBounds, WindowBounds,
    };

    fn window_candidate(
        raw_hwnd: isize,
        window_class: &str,
        window_title: &str,
        width: i32,
        height: i32,
    ) -> WindowBounds {
        WindowBounds {
            raw_hwnd,
            pid: 42,
            visible: true,
            minimized: false,
            is_tool_window: false,
            window_class: window_class.to_string(),
            window_title: window_title.to_string(),
            x: 0,
            y: 0,
            width,
            height,
            client_bounds: Some(BoundsSnapshot {
                x: 0,
                y: 0,
                width,
                height,
            }),
            display_id: Some("DISPLAY-1".to_string()),
            style: None,
        }
    }

    #[test]
    fn accepts_bounds_that_cover_the_target_display() {
        let expected = BoundsSnapshot {
            x: 1920,
            y: 0,
            width: 1920,
            height: 1080,
        };
        let detected = BoundsSnapshot {
            x: 1921,
            y: 0,
            width: 1919,
            height: 1080,
        };

        assert!(bounds_cover_target(&detected, &expected));
    }

    #[test]
    fn rejects_windowed_bounds_inside_the_target_display() {
        let expected = BoundsSnapshot {
            x: 1920,
            y: 0,
            width: 1920,
            height: 1080,
        };
        let detected = BoundsSnapshot {
            x: 2100,
            y: 120,
            width: 800,
            height: 600,
        };

        assert!(!bounds_cover_target(&detected, &expected));
    }

    #[test]
    fn ona_compatible_game_without_game_ready_cannot_use_legacy_fallback() {
        assert!(!should_return_legacy_fallback(true, true));
    }

    #[test]
    fn legacy_exe_without_lifecycle_can_use_legacy_fallback_after_timeout() {
        assert!(should_return_legacy_fallback(true, false));
    }

    #[test]
    fn rejects_winit_event_target_and_tiny_helper_windows() {
        let winit_helper = window_candidate(0xf606d0, "Winit Thread Event Target", "", 16, 16);
        let tiny_helper = window_candidate(0x22, "HelperWindow", "", 16, 16);

        assert_eq!(
            primary_candidate_rejection_reason(&winit_helper),
            Some("HELPER_CLASS")
        );
        assert_eq!(
            primary_candidate_rejection_reason(&tiny_helper),
            Some("WINDOW_TOO_SMALL")
        );
    }

    #[test]
    fn fullscreen_primary_candidate_outranks_auxiliary_window() {
        let target = TargetDisplayBounds {
            display_id: "DISPLAY-1".to_string(),
            x: 0,
            y: 0,
            width: 1366,
            height: 768,
            presentation_mode: "CONSOLE_FULLSCREEN".to_string(),
        };
        let primary = window_candidate(0x1d0986, "Window Class", "Game", 1366, 768);
        let auxiliary = window_candidate(0xf606d0, "Winit Thread Event Target", "", 16, 16);

        assert!(primary_candidate_rejection_reason(&primary).is_none());
        assert!(primary_candidate_rejection_reason(&auxiliary).is_some());
        assert!(window_candidate_score(&primary, Some(&target), None) > 1_000_000);
    }

    #[test]
    fn persisted_identity_keeps_the_captured_hwnd() {
        let stored = StoredPrimaryGameWindow {
            game_id: Some("game-id".to_string()),
            pid: 42,
            raw_hwnd: 0x1d0986,
            window_class: "Window Class".to_string(),
            window_title: "Game".to_string(),
            expected_bounds: BoundsSnapshot {
                x: 0,
                y: 0,
                width: 1366,
                height: 768,
            },
            expected_monitor: "DISPLAY-1".to_string(),
        };

        assert_eq!(stored.identity().hwnd, "0x1d0986");
    }

    #[test]
    fn restore_commit_requires_a_visible_non_minimized_compatible_window() {
        assert!(restore_commit_allowed(true, true, false, true, true, true));
        assert!(!restore_commit_allowed(
            true, false, false, true, true, true
        ));
        assert!(!restore_commit_allowed(true, true, true, true, true, true));
        assert!(!restore_commit_allowed(
            true, true, false, false, true, true
        ));
        assert!(!restore_commit_allowed(
            true, true, false, true, false, true
        ));
        assert!(!restore_commit_allowed(
            true, true, false, true, true, false
        ));
    }
}
