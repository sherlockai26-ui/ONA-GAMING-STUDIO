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
    pub foreground_attempted: bool,
    pub foreground_granted: bool,
}

pub fn wait_for_game_handoff_ready(
    pid: u32,
    target: TargetDisplayBounds,
    timeout_ms: u64,
    mut handshake_display_ready: impl FnMut() -> bool,
    mut handshake_game_ready: impl FnMut() -> bool,
) -> GameHandoffStatus {
    let timeout = Duration::from_millis(timeout_ms.max(1));
    let started = Instant::now();
    let mut window_ready = false;
    let mut window_on_target_display = false;
    let mut handshake_confirmed = false;
    let mut game_ready = false;
    let mut foreground_attempted = false;
    let mut foreground_granted = false;

    while started.elapsed() < timeout {
        if handshake_display_ready() {
            handshake_confirmed = true;
        }

        if handshake_game_ready() {
            handshake_confirmed = true;
            game_ready = true;
        }

        if let Some(window) = platform_visible_window_for_pid(pid) {
            window_ready = true;
            window_on_target_display = window_is_on_target_display(&window, &target);

            if window_on_target_display && !foreground_attempted {
                foreground_attempted = true;
                foreground_granted = platform_try_foreground_window(&window);
            }

            if window_on_target_display && game_ready {
                return GameHandoffStatus {
                    process_ready: true,
                    window_ready,
                    window_on_target_display,
                    display_ready: true,
                    game_ready,
                    handshake_confirmed,
                    legacy_fallback: false,
                    foreground_attempted,
                    foreground_granted,
                };
            }

            if window_on_target_display && !handshake_confirmed {
                return GameHandoffStatus {
                    process_ready: true,
                    window_ready,
                    window_on_target_display,
                    display_ready: false,
                    game_ready: false,
                    handshake_confirmed: false,
                    legacy_fallback: true,
                    foreground_attempted,
                    foreground_granted,
                };
            }
        }

        thread::sleep(POLL_INTERVAL);
    }

    GameHandoffStatus {
        process_ready: true,
        window_ready,
        window_on_target_display,
        display_ready: false,
        game_ready,
        handshake_confirmed,
        legacy_fallback: false,
        foreground_attempted,
        foreground_granted,
    }
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

#[derive(Clone, Debug)]
struct WindowBounds {
    #[cfg(windows)]
    hwnd: windows::Win32::Foundation::HWND,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    display_id: Option<String>,
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
                    display_id: display_id_for_window(window),
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
