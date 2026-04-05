use std::mem;

use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

struct EnumData {
    windows: Vec<HWND>,
    monitor_rect: RECT,
}

fn rects_intersect(a: &RECT, b: &RECT) -> bool {
    !(a.right <= b.left || a.left >= b.right || a.bottom <= b.top || a.top >= b.bottom)
}

unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let data = &mut *(lparam.0 as *mut EnumData);

    if super::is_app_window(hwnd) {
        let mut window_rect: RECT = mem::zeroed();
        if GetWindowRect(hwnd, &mut window_rect).is_ok() {
            if rects_intersect(&window_rect, &data.monitor_rect) {
                data.windows.push(hwnd);
            }
        }
    }
    BOOL(1)
}

pub unsafe fn collect_app_windows(monitor_rect: &RECT) -> Vec<HWND> {
    let mut data = EnumData {
        windows: Vec::new(),
        monitor_rect: *monitor_rect,
    };

    let _ = EnumWindows(
        Some(enum_callback),
        LPARAM(&mut data as *mut EnumData as isize),
    );

    data.windows
}
