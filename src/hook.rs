use std::sync::atomic::{AtomicUsize, Ordering};

use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::debug;

const VK_LWIN: u32 = 0x5B;
const VK_RWIN: u32 = 0x5C;
const VK_D: u32 = 0x44;

static mut HHOOK: Option<HHOOK> = None;
static mut WIN_HELD: bool = false;
static mut CONSUMED: bool = false;
static mut WIN_PASSTHROUGH: bool = false;
static mut WIN_VK: u8 = 0;
static TARGET_HWND: AtomicUsize = AtomicUsize::new(0);

unsafe extern "system" fn low_level_keyboard_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code >= 0 {
        let kb = &*(l_param.0 as *const KBDLLHOOKSTRUCT);
        let vk = kb.vkCode;

        if (kb.flags & KBDLLHOOKSTRUCT_FLAGS(0x10)) != KBDLLHOOKSTRUCT_FLAGS(0) {
            return CallNextHookEx(HHOOK, n_code, w_param, l_param);
        }

        let is_down =
            w_param.0 == WM_KEYDOWN as usize || w_param.0 == WM_SYSKEYDOWN as usize;
        let is_up =
            w_param.0 == WM_KEYUP as usize || w_param.0 == WM_SYSKEYUP as usize;

        match vk {
            VK_LWIN | VK_RWIN => {
                if is_down {
                    WIN_HELD = true;
                    WIN_VK = vk as u8;
                    CONSUMED = false;
                    WIN_PASSTHROUGH = false;
                    return LRESULT(1);
                } else if is_up {
                    WIN_HELD = false;
                    if CONSUMED {
                        CONSUMED = false;
                        return LRESULT(1);
                    }
                    if WIN_PASSTHROUGH {
                        WIN_PASSTHROUGH = false;
                        return CallNextHookEx(HHOOK, n_code, w_param, l_param);
                    }
                    keybd_event(WIN_VK, 0, KEYBD_EVENT_FLAGS(0), 0);
                    keybd_event(WIN_VK, 0, KEYEVENTF_KEYUP, 0);
                    return LRESULT(1);
                }
            }
            VK_D => {
                if WIN_HELD && is_down {
                    debug::log("WIN+D -> posting toggle");
                    CONSUMED = true;
                    let hwnd_usize = TARGET_HWND.load(Ordering::Relaxed);
                    let hwnd = HWND(hwnd_usize as *mut std::ffi::c_void);
                    let _ = PostMessageW(
                        Some(hwnd),
                        crate::WM_TRIGGER_TOGGLE,
                        WPARAM(0),
                        LPARAM(0),
                    );
                    return LRESULT(1);
                }
                if CONSUMED && is_up {
                    return LRESULT(1);
                }
            }
            _ => {
                if WIN_HELD && !CONSUMED && !WIN_PASSTHROUGH && is_down {
                    WIN_PASSTHROUGH = true;
                    keybd_event(WIN_VK, 0, KEYBD_EVENT_FLAGS(0), 0);
                }
            }
        }
    }

    CallNextHookEx(HHOOK, n_code, w_param, l_param)
}

pub unsafe fn install(hinstance: super::HMODULE, hwnd: super::HWND) {
    TARGET_HWND.store(hwnd.0 as usize, Ordering::Relaxed);

    match SetWindowsHookExW(
        WH_KEYBOARD_LL,
        Some(low_level_keyboard_proc),
        Some(hinstance.into()),
        0,
    ) {
        Ok(h) => {
            debug::log("keyboard hook installed successfully");
            HHOOK = Some(h);
        }
        Err(_) => {
            debug::log("FAILED to install keyboard hook");
            return;
        }
    }
}

pub unsafe fn uninstall() {
    if let Some(h) = HHOOK {
        let _ = UnhookWindowsHookEx(h);
        HHOOK = None;
    }
}
