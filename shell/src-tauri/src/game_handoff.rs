use std::{
    thread,
    time::{Duration, Instant},
};

const POLL_INTERVAL: Duration = Duration::from_millis(80);

#[derive(Clone, Debug)]
pub struct TargetDisplayBounds {
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
    pub handshake_confirmed: bool,
    pub legacy_fallback: bool,
}

pub fn wait_for_game_handoff_ready(
    pid: u32,
    target: TargetDisplayBounds,
    timeout_ms: u64,
    mut handshake_display_ready: impl FnMut() -> bool,
) -> GameHandoffStatus {
    let timeout = Duration::from_millis(timeout_ms.max(1));
    let started = Instant::now();
    let mut window_ready = false;
    let mut window_on_target_display = false;

    while started.elapsed() < timeout {
        if handshake_display_ready() {
            return GameHandoffStatus {
                process_ready: true,
                window_ready: true,
                window_on_target_display: true,
                display_ready: true,
                handshake_confirmed: true,
                legacy_fallback: false,
            };
        }

        if let Some(window) = platform_visible_window_for_pid(pid) {
            window_ready = true;
            window_on_target_display = window_intersects_target_display(&window, &target);

            if window_on_target_display {
                return GameHandoffStatus {
                    process_ready: true,
                    window_ready,
                    window_on_target_display,
                    display_ready: false,
                    handshake_confirmed: false,
                    legacy_fallback: true,
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
        handshake_confirmed: false,
        legacy_fallback: false,
    }
}

fn window_intersects_target_display(window: &WindowBounds, target: &TargetDisplayBounds) -> bool {
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
    x: i32,
    y: i32,
    width: i32,
    height: i32,
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
                    x: rect.left,
                    y: rect.top,
                    width: rect.right - rect.left,
                    height: rect.bottom - rect.top,
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

#[cfg(not(windows))]
fn platform_visible_window_for_pid(_pid: u32) -> Option<WindowBounds> {
    None
}
