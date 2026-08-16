use std::sync::atomic::{AtomicBool, Ordering};

static GAME_CURSOR_HIDDEN: AtomicBool = AtomicBool::new(false);

pub fn hide_for_game_session(window: Option<&tauri::WebviewWindow>) {
    if GAME_CURSOR_HIDDEN.swap(true, Ordering::SeqCst) {
        return;
    }

    if let Some(window) = window {
        let _ = window.set_cursor_visible(false);
    }

    platform_hide_cursor();
    println!("[ONA Cursor] Hidden for native game presentation.");
}

pub fn restore_after_game_session(window: Option<&tauri::WebviewWindow>) {
    if !GAME_CURSOR_HIDDEN.swap(false, Ordering::SeqCst) {
        return;
    }

    platform_restore_cursor();

    if let Some(window) = window {
        let _ = window.set_cursor_visible(true);
    }

    println!("[ONA Cursor] Restored after native game presentation.");
}

#[cfg(windows)]
fn platform_hide_cursor() {
    use windows::Win32::UI::WindowsAndMessaging::ShowCursor;

    unsafe {
        for _ in 0..16 {
            if ShowCursor(false) < 0 {
                break;
            }
        }
    }
}

#[cfg(not(windows))]
fn platform_hide_cursor() {}

#[cfg(windows)]
fn platform_restore_cursor() {
    use windows::Win32::UI::WindowsAndMessaging::ShowCursor;

    unsafe {
        for _ in 0..16 {
            if ShowCursor(true) >= 0 {
                break;
            }
        }
    }
}

#[cfg(not(windows))]
fn platform_restore_cursor() {}
