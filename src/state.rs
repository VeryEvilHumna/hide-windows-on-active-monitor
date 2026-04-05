use std::collections::HashMap;
use std::sync::Mutex;

use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::w;

use crate::debug;

fn lock_states<'a>() -> std::sync::MutexGuard<'a, Option<HashMap<String, Vec<isize>>>> {
    match STATES.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    }
}

static STATES: Mutex<Option<HashMap<String, Vec<isize>>>> = Mutex::new(None);

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
    let mut guard = lock_states();
    let states = guard.get_or_insert_with(HashMap::new);
    if let Some(windows) = states.get_mut(monitor_name) {
        windows.retain(|&hwnd| IsWindow(Some(HWND(hwnd as *mut _))).as_bool());
        debug::log(&format!("is_hidden('{}'): {} alive windows after pruning", monitor_name, windows.len()));
        !windows.is_empty()
    } else {
        debug::log(&format!("is_hidden('{}'): no entry in state", monitor_name));
        false
    }
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

    let mut guard = lock_states();
    let states = guard.get_or_insert_with(HashMap::new);
    states.insert(
        monitor_name.to_string(),
        windows.iter().map(|h| h.0 as isize).collect(),
    );
    debug::log(&format!("hide('{}'): state saved", monitor_name));
}

pub unsafe fn restore(monitor_name: &str) {
    debug::log(&format!("restore('{}')", monitor_name));
    let to_restore: Vec<HWND> = {
        let mut guard = lock_states();
        let states = guard.get_or_insert_with(HashMap::new);
        let stored = states.remove(monitor_name).unwrap_or_default();
        debug::log(&format!("  stored {} hwnds, filtering alive...", stored.len()));
        stored
            .into_iter()
            .filter(|&hwnd| IsWindow(Some(HWND(hwnd as *mut _))).as_bool())
            .map(|hwnd| HWND(hwnd as *mut _))
            .collect()
    };

    debug::log(&format!("  {} windows to restore (reverse z-order)", to_restore.len()));
    for hwnd in to_restore.iter().rev() {
        debug::log(&format!("  ShowWindow SW_RESTORE hwnd={:?}", hwnd.0));
        let result = ShowWindow(*hwnd, SW_RESTORE);
        debug::log(&format!("  ShowWindow result={}", result.0));
    }

    if let Some(&topmost) = to_restore.first() {
        force_set_foreground(topmost);
        debug::log(&format!("  force_set_foreground topmost hwnd={:?}", topmost.0));
    }
}

pub unsafe fn restore_stale_entries(current_monitor: &str) {
    let stale: Vec<(String, Vec<isize>)> = {
        let mut guard = lock_states();
        let states = guard.get_or_insert_with(HashMap::new);
        let mut to_restore = Vec::new();
        let keys: Vec<String> = states.keys().cloned().collect();
        for key in keys {
            if key == current_monitor {
                continue;
            }
            if let Some(windows) = states.remove(&key) {
                let alive: Vec<isize> = windows
                    .into_iter()
                    .filter(|&hwnd| IsWindow(Some(HWND(hwnd as *mut _))).as_bool())
                    .collect();
                if !alive.is_empty() {
                    debug::log(&format!("restore_stale: found {} orphaned windows from '{}'", alive.len(), key));
                    to_restore.push((key, alive));
                }
            }
        }
        to_restore
    };

    for (_name, hwnds) in stale {
        for &hwnd_raw in hwnds.iter().rev() {
            let hwnd = HWND(hwnd_raw as *mut _);
            debug::log(&format!("  restoring orphaned hwnd={:?}", hwnd.0));
            let _ = ShowWindow(hwnd, SW_RESTORE);
        }
    }
}
