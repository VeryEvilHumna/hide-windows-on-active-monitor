use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};

use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::debug;

const VK_LWIN: u32 = 0x5B;
const VK_RWIN: u32 = 0x5c;
const VK_D: u32 = 0x44;

static HOOK: AtomicUsize = AtomicUsize::new(0);
static WIN_HELD: AtomicBool = AtomicBool::new(false);
static CONSUMED: AtomicBool = AtomicBool::new(false);
static WIN_PASSTHROUGH: AtomicBool = AtomicBool::new(false);
static WIN_VK: AtomicU8 = AtomicU8::new(0);
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
            let hook = HOOK.load(Ordering::Relaxed);
            return CallNextHookEx(
                if hook == 0 { None } else { Some(HHOOK(hook as *mut _)) },
                n_code,
                w_param,
                l_param,
            );
        }

        let is_down =
            w_param.0 == WM_KEYDOWN as usize || w_param.0 == WM_SYSKEYDOWN as usize;
        let is_up =
            w_param.0 == WM_KEYUP as usize || w_param.0 == WM_SYSKEYUP as usize;

        let hook = HOOK.load(Ordering::Relaxed);
        let hhook = if hook == 0 { None } else { Some(HHOOK(hook as *mut _)) };

        match vk {
            VK_LWIN | VK_RWIN => {
                if is_down {
                    WIN_HELD.store(true, Ordering::Relaxed);
                    WIN_VK.store(vk as u8, Ordering::Relaxed);
                    CONSUMED.store(false, Ordering::Relaxed);
                    WIN_PASSTHROUGH.store(false, Ordering::Relaxed);
                    return LRESULT(1);
                } else if is_up {
                    WIN_HELD.store(false, Ordering::Relaxed);
                    if CONSUMED.swap(false, Ordering::Relaxed) {
                        return LRESULT(1);
                    }
                    if WIN_PASSTHROUGH.swap(false, Ordering::Relaxed) {
                        return CallNextHookEx(hhook, n_code, w_param, l_param);
                    }
                    let win_vk = WIN_VK.load(Ordering::Relaxed);
                    keybd_event(win_vk, 0, KEYBD_EVENT_FLAGS(0), 0);
                    keybd_event(win_vk, 0, KEYEVENTF_KEYUP, 0);
                    return LRESULT(1);
                }
            }
            VK_D => {
                if WIN_HELD.load(Ordering::Relaxed) && is_down {
                    debug::log("WIN+D -> posting toggle");
                    CONSUMED.store(true, Ordering::Relaxed);
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
                if CONSUMED.load(Ordering::Relaxed) && is_up {
                    return LRESULT(1);
                }
            }
            _ => {
                if WIN_HELD.load(Ordering::Relaxed)
                    && !CONSUMED.load(Ordering::Relaxed)
                    && !WIN_PASSTHROUGH.load(Ordering::Relaxed)
                    && is_down
                {
                    WIN_PASSTHROUGH.store(true, Ordering::Relaxed);
                    let win_vk = WIN_VK.load(Ordering::Relaxed);
                    keybd_event(win_vk, 0, KEYBD_EVENT_FLAGS(0), 0);
                }
            }
        }
    }

    let hook = HOOK.load(Ordering::Relaxed);
    let hhook = if hook == 0 { None } else { Some(HHOOK(hook as *mut _)) };
    CallNextHookEx(hhook, n_code, w_param, l_param)
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
            HOOK.store(h.0 as usize, Ordering::Relaxed);
        }
        Err(_) => {
            debug::log("FAILED to install keyboard hook");
        }
    }
}

pub unsafe fn uninstall() {
    let hook = HOOK.swap(0, Ordering::Relaxed);
    if hook != 0 {
        let _ = UnhookWindowsHookEx(HHOOK(hook as *mut _));
    }
}
