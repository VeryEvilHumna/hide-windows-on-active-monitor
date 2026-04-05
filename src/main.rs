#![windows_subsystem = "windows"]

use std::mem;
use std::sync::atomic::{AtomicPtr, Ordering};

use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Dwm::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

mod autostart;
mod debug;
mod hook;
mod state;
mod tray;
mod window;

const WM_TRAYICON: u32 = WM_USER + 1;
const WM_TRIGGER_TOGGLE: u32 = WM_USER + 2;
const ID_MENU_AUTOSTART: u32 = 1001;
const ID_MENU_EXIT: u32 = 1002;

static OWN_HWND: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());
static INSTANCE_MUTEX: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());

unsafe fn is_app_window(hwnd: HWND) -> bool {
    if HWND(OWN_HWND.load(Ordering::Relaxed)) == hwnd {
        return false;
    }
    if IsWindowVisible(hwnd).0 == 0 {
        return false;
    }
    if IsIconic(hwnd).as_bool() {
        return false;
    }
    if GetWindowTextLengthW(hwnd) == 0 {
        return false;
    }

    let style = GetWindowLongW(hwnd, GWL_STYLE);
    if (style & WS_CHILD.0 as i32) != 0 {
        return false;
    }
    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
    if (ex_style & WS_EX_TOOLWINDOW.0 as i32) != 0 {
        return false;
    }

    let mut buf: [u16; 256] = [0; 256];
    GetClassNameW(hwnd, &mut buf);
    let class_name = String::from_utf16_lossy(
        &buf.iter().take_while(|&&c| c != 0).copied().collect::<Vec<_>>(),
    );
    let skip_classes = [
        "Shell_TrayWnd",
        "Shell_SecondaryTrayWnd",
        "Progman",
        "WorkerW",
        "WindowsDashboard",
    ];
    if skip_classes.iter().any(|&s| class_name == s) {
        return false;
    }

    let mut cloaked: u32 = 0;
    if DwmGetWindowAttribute(
        hwnd,
        DWMWA_CLOAKED,
        &mut cloaked as *mut _ as *mut std::ffi::c_void,
        std::mem::size_of::<u32>() as u32,
    )
    .is_ok()
        && cloaked != 0
    {
        return false;
    }

    debug::log(&format!("is_app_window PASS: hwnd={:?} class='{}'", hwnd.0, class_name));

    true
}

unsafe fn is_desktop_window(hwnd: HWND) -> bool {
    let mut buf: [u16; 256] = [0; 256];
    GetClassNameW(hwnd, &mut buf);
    let class_name = String::from_utf16_lossy(
        &buf.iter().take_while(|&&c| c != 0).copied().collect::<Vec<_>>(),
    );
    class_name == "Progman" || class_name == "WorkerW"
}

unsafe fn get_foreground_window_monitor() -> Option<HMONITOR> {
    let fg = GetForegroundWindow();
    if fg.is_invalid() {
        return None;
    }
    if is_desktop_window(fg) {
        return None;
    }
    let hmon = MonitorFromWindow(fg, MONITOR_DEFAULTTONEAREST);
    if hmon.is_invalid() {
        return None;
    }
    Some(hmon)
}

unsafe fn get_cursor_monitor() -> Option<HMONITOR> {
    let mut point = POINT { x: 0, y: 0 };
    if GetCursorPos(&mut point).is_err() {
        return None;
    }
    let hmon = MonitorFromPoint(point, MONITOR_DEFAULTTONEAREST);
    if hmon.is_invalid() {
        return None;
    }
    Some(hmon)
}

unsafe fn get_monitor_device_name(hmon: HMONITOR) -> String {
    let mut info: MONITORINFOEXW = mem::zeroed();
    info.monitorInfo.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
    if GetMonitorInfoW(hmon, &mut info as *mut MONITORINFOEXW as *mut MONITORINFO).0 == 0 {
        return String::new();
    }
    String::from_utf16_lossy(
        &info.szDevice.iter().take_while(|&&c| c != 0).copied().collect::<Vec<_>>(),
    )
}

unsafe fn get_monitor_rect(hmon: HMONITOR) -> Option<RECT> {
    let mut info: MONITORINFOEXW = mem::zeroed();
    info.monitorInfo.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
    if GetMonitorInfoW(hmon, &mut info as *mut MONITORINFOEXW as *mut MONITORINFO).0 == 0 {
        return None;
    }
    Some(info.monitorInfo.rcMonitor)
}

unsafe fn perform_toggle() {
    debug::log("perform_toggle called");

    let current_monitor =
        get_foreground_window_monitor().unwrap_or_else(|| {
            debug::log("no foreground window monitor, falling back to cursor");
            get_cursor_monitor().unwrap_or(HMONITOR(std::ptr::null_mut()))
        });

    if current_monitor.0.is_null() {
        debug::log("ABORT: could not determine current monitor");
        return;
    }

    let monitor_name = get_monitor_device_name(current_monitor);
    if monitor_name.is_empty() {
        debug::log("ABORT: empty monitor device name");
        return;
    }
    debug::log(&format!("monitor: {}", monitor_name));

    let monitor_rect = match get_monitor_rect(current_monitor) {
        Some(r) => {
            debug::log(&format!("monitor rect: ({}, {})-({}, {})", r.left, r.top, r.right, r.bottom));
            r
        }
        None => {
            debug::log("ABORT: could not get monitor rect");
            return;
        }
    };

    if state::is_hidden(&monitor_name) {
        debug::log(&format!("restoring windows on {}", monitor_name));
        state::restore(&monitor_name);
    } else {
        state::restore_stale_entries(&monitor_name);
        debug::log(&format!("hiding windows on {}", monitor_name));
        let windows = window::collect_app_windows(&monitor_rect);
        if windows.is_empty() {
            debug::log("ABORT: no app windows found on this monitor");
            return;
        }
        debug::log(&format!("found {} windows to hide", windows.len()));
        state::hide(&monitor_name, &windows);
    }
}

unsafe extern "system" fn wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            tray::remove(hwnd);
            hook::uninstall();
            let mutex_ptr = INSTANCE_MUTEX.swap(std::ptr::null_mut(), Ordering::Relaxed);
            if !mutex_ptr.is_null() {
                let _ = CloseHandle(HANDLE(mutex_ptr));
            }
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_TRIGGER_TOGGLE => {
            perform_toggle();
            LRESULT(0)
        }
        WM_TRAYICON => {
            let event = lparam.0 as u32;
            debug::log(&format!("WM_TRAYICON event={:#X}", event));
            if event == WM_RBUTTONUP as u32 {
                tray::show_context_menu(hwnd, autostart::is_enabled());
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            let id = (wparam.0 & 0xFFFF) as u32;
            debug::log(&format!("WM_COMMAND id={}", id));
            match id {
                ID_MENU_AUTOSTART => {
                    if autostart::is_enabled() {
                        autostart::disable();
                    } else {
                        autostart::enable();
                    }
                }
                ID_MENU_EXIT => {
                    let _ = DestroyWindow(hwnd);
                }
                _ => {}
            }
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn ensure_single_instance() -> Option<HANDLE> {
    match CreateMutexW(None, false, w!("HideWinHide_Mutex")) {
        Ok(mutex) => {
            if GetLastError() == ERROR_ALREADY_EXISTS {
                let _ = CloseHandle(mutex);
                return None;
            }
            Some(mutex)
        }
        Err(_) => None,
    }
}

fn main() -> windows::core::Result<()> {
    unsafe {
        let mutex = match ensure_single_instance() {
            Some(h) => h,
            None => return Ok(()),
        };
        INSTANCE_MUTEX.store(mutex.0, Ordering::Relaxed);

        let hinstance = GetModuleHandleW(None)?;

        let wc = WNDCLASSW {
            lpfnWndProc: Some(wndproc),
            hInstance: hinstance.into(),
            lpszClassName: w!("HideWinHide"),
            ..Default::default()
        };

        if RegisterClassW(&wc) == 0 {
            debug::log("RegisterClassW FAILED");
            return Err(windows::core::Error::from_win32());
        }

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("HideWinHide"),
            None,
            WINDOW_STYLE::default(),
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            Some(HWND_MESSAGE),
            None,
            Some(hinstance.into()),
            None,
        )?;

        OWN_HWND.store(hwnd.0, Ordering::Relaxed);

        hook::install(hinstance, hwnd);
        tray::create(hwnd);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        Ok(())
    }
}
