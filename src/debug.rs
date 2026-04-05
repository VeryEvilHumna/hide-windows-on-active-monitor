use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use windows::core::*;
use windows::Win32::System::Diagnostics::Debug::*;

pub fn log(msg: &str) {
    let wide: Vec<u16> = OsStr::new(&format!("[HideWin] {}\0", msg))
        .encode_wide()
        .collect();
    unsafe {
        OutputDebugStringW(PCWSTR(wide.as_ptr()));
    }
}
