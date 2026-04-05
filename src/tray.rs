use std::ptr;

use windows::Win32::Foundation::*;
use windows::Win32::UI::Shell::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::{w, PCWSTR};

use crate::debug;

unsafe fn append_menu_item(menu: HMENU, id: u32, flags: MENU_ITEM_FLAGS, text: &str) {
    let wtext: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    AppendMenuW(menu, flags, id as usize, PCWSTR(wtext.as_ptr())).ok();
}

pub unsafe fn create(hwnd: HWND) {
    let tooltip = w!("Win+D: Per-monitor hide\0");

    let icon = LoadIconW(None, IDI_APPLICATION).unwrap_or_default();

    let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1;
    nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
    nid.uCallbackMessage = super::WM_TRAYICON;
    nid.hIcon = icon;
    let copy_len = tooltip.as_wide().len().min(nid.szTip.len());
    nid.szTip[..copy_len].copy_from_slice(&tooltip.as_wide()[..copy_len]);

    let result = Shell_NotifyIconW(NIM_ADD, &nid);
    if result.0 != 0 {
        debug::log("tray icon created");
    } else {
        debug::log(&format!("tray icon FAILED, result={}", result.0));
    }
}

pub unsafe fn remove() {
    let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.uID = 1;
    let _ = Shell_NotifyIconW(NIM_DELETE, &nid);
}

pub unsafe fn show_context_menu(hwnd: HWND, autostart_enabled: bool) {
    debug::log("show_context_menu called");

    let menu = match CreatePopupMenu() {
        Ok(m) => m,
        Err(_) => {
            debug::log("CreatePopupMenu FAILED");
            return;
        }
    };

    let flags = if autostart_enabled {
        MF_CHECKED | MF_STRING
    } else {
        MF_UNCHECKED | MF_STRING
    };
    append_menu_item(menu, super::ID_MENU_AUTOSTART, flags, "Start with Windows");

    AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR(ptr::null())).ok();
    append_menu_item(menu, super::ID_MENU_EXIT, MF_STRING, "Exit");

    let mut point = POINT { x: 0, y: 0 };
    let _ = GetCursorPos(&mut point);
    let _ = SetForegroundWindow(hwnd);
    let _ = TrackPopupMenu(menu, TPM_RIGHTALIGN | TPM_BOTTOMALIGN, point.x, point.y, None, hwnd, None);
    let _ = PostMessageW(Some(hwnd), WM_NULL, WPARAM(0), LPARAM(0));
    DestroyMenu(menu).ok();
    debug::log("show_context_menu done");
}
