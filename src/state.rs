use std::collections::HashMap;

use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::w;

use crate::debug;

static mut STATES: Option<HashMap<String, Vec<isize>>> = None;

unsafe fn with_states<F, R>(f: F) -> R
where
    F: FnOnce(&mut HashMap<String, Vec<isize>>) -> R,
{
    if STATES.is_none() {
        STATES = Some(HashMap::new());
    }
    f(STATES.as_mut().unwrap_unchecked())
}

unsafe fn force_set_foreground(hwnd: HWND) {
    let fg = GetForegroundWindow();
    if fg.is_invalid() {
        let _ = SetForegroundWindow(hwnd);
        return;
    }
    let fg_tid = GetWindowThreadProcessId(fg, None);
    let cur_tid = GetCurrentThreadId();
    let _ = AttachThreadInput(cur_tid, fg_tid, true);
    let _ = BringWindowToTop(hwnd);
    let _ = SetForegroundWindow(hwnd);
    let _ = AttachThreadInput(cur_tid, fg_tid, false);
}

pub unsafe fn is_hidden(monitor_name: &str) -> bool {
    with_states(|s: &mut HashMap<String, Vec<isize>>| {
        if let Some(windows) = s.get(monitor_name) {
            let any_alive = windows
                .iter()
                .copied()
                .any(|hwnd| IsWindow(Some(HWND(hwnd as *mut _))).as_bool());
            debug::log(&format!("is_hidden('{}'): stored={} windows, any_alive={}", monitor_name, windows.len(), any_alive));
            any_alive
        } else {
            debug::log(&format!("is_hidden('{}'): no entry in state", monitor_name));
            false
        }
    })
}

pub unsafe fn hide(monitor_name: &str, windows: &[HWND]) {
    debug::log(&format!("hide('{}'): {} windows to minimize", monitor_name, windows.len()));
    for &hwnd in windows {
        let result = ShowWindow(hwnd, SW_SHOWMINNOACTIVE);
        debug::log(&format!("  ShowWindow SW_SHOWMINNOACTIVE hwnd={:?} result={}", hwnd.0, result.0));
    }

    if let Ok(progman) = FindWindowW(w!("Progman"), None) {
        force_set_foreground(progman);
        debug::log("force_set_foreground(Progman)");
    }

    with_states(|s: &mut HashMap<String, Vec<isize>>| {
        s.insert(
            monitor_name.to_string(),
            windows.iter().map(|h| h.0 as isize).collect(),
        );
    });
    debug::log(&format!("hide('{}'): state saved", monitor_name));
}

pub unsafe fn restore(monitor_name: &str) {
    debug::log(&format!("restore('{}')", monitor_name));
    let to_restore: Vec<HWND> = with_states(|s: &mut HashMap<String, Vec<isize>>| {
        let stored = s.remove(monitor_name).unwrap_or_default();
        debug::log(&format!("  stored {} hwnds, filtering alive...", stored.len()));
        stored
            .into_iter()
            .filter(|&hwnd| IsWindow(Some(HWND(hwnd as *mut _))).as_bool())
            .map(|hwnd| HWND(hwnd as *mut _))
            .collect()
    });

    debug::log(&format!("  {} windows to restore (reverse z-order)", to_restore.len()));
    for hwnd in to_restore.iter().rev() {
        debug::log(&format!("  ShowWindow SW_RESTORE hwnd={:?}", hwnd.0));
        let result = ShowWindow(*hwnd, SW_RESTORE);
        debug::log(&format!("  ShowWindow result={}", result.0));
    }
}
