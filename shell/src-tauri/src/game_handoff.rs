use std::{
    thread,
    time::{Duration, Instant},
};

const POLL_INTERVAL: Duration = Duration::from_millis(80);

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

        if let Some(window) = platform_visible_window_for_pid(pid) {
            window_ready = true;
            window_on_target_display = window_is_on_target_display(&window, &target);
            let presentation = validate_window_presentation(&window, &target);
            last_presentation_valid = presentation.valid;
            last_diagnostics =
                handoff_diagnostics(&target, Some(&presentation), game_ready, ona_compatible);

            if window_on_target_display && !foreground_attempted {
                foreground_attempted = true;
                foreground_granted = platform_try_foreground_window(&window);
            }

            if game_ready && presentation.valid {
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
                };
            }
        }

        thread::sleep(POLL_INTERVAL);
    }

    if should_return_legacy_fallback(last_presentation_valid, ona_compatible) {
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
    }
}

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
    #[cfg(windows)]
    hwnd: windows::Win32::Foundation::HWND,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    client_bounds: Option<BoundsSnapshot>,
    display_id: Option<String>,
    style: Option<WindowStyleSnapshot>,
}

#[cfg(windows)]
fn platform_visible_window_for_pid(pid: u32) -> Option<WindowBounds> {
    use windows::Win32::{
        Foundation::{BOOL, HWND, LPARAM, RECT},
        UI::WindowsAndMessaging::{
            EnumWindows, GetWindow, GetWindowRect, GetWindowThreadProcessId, IsWindowVisible,
            GW_OWNER,
        },
    };

    struct SearchState {
        pid: u32,
        bounds: Option<WindowBounds>,
    }

    unsafe extern "system" fn enum_window(window: HWND, lparam: LPARAM) -> BOOL {
        let state = &mut *(lparam.0 as *mut SearchState);
        let mut window_pid = 0;

        GetWindowThreadProcessId(window, Some(&mut window_pid));

        let is_unowned = GetWindow(window, GW_OWNER)
            .map(|owner| owner.0 == std::ptr::null_mut())
            .unwrap_or(true);

        if window_pid == state.pid && IsWindowVisible(window).as_bool() && is_unowned {
            let mut rect = RECT::default();

            if GetWindowRect(window, &mut rect).is_ok() {
                state.bounds = Some(WindowBounds {
                    hwnd: window,
                    x: rect.left,
                    y: rect.top,
                    width: rect.right - rect.left,
                    height: rect.bottom - rect.top,
                    client_bounds: client_bounds_for_window(window),
                    display_id: display_id_for_window(window),
                    style: style_for_window(window),
                });
            }

            return BOOL(0);
        }

        BOOL(1)
    }

    let mut state = SearchState { pid, bounds: None };

    unsafe {
        let _ = EnumWindows(Some(enum_window), LPARAM(&mut state as *mut _ as isize));
    }

    state.bounds
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
    use windows::Win32::UI::WindowsAndMessaging::{SetForegroundWindow, ShowWindow, SW_RESTORE};

    unsafe {
        let _ = ShowWindow(window.hwnd, SW_RESTORE);
        SetForegroundWindow(window.hwnd).as_bool()
    }
}

#[cfg(not(windows))]
fn platform_try_foreground_window(_window: &WindowBounds) -> bool {
    false
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
fn platform_visible_window_for_pid(_pid: u32) -> Option<WindowBounds> {
    None
}

#[cfg(test)]
mod tests {
    use super::should_return_legacy_fallback;
    use super::{bounds_cover_target, BoundsSnapshot};

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
}
