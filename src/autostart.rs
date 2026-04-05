use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::Registry::*;
use windows::core::*;

pub unsafe fn is_enabled() -> bool {
    let mut key: HKEY = HKEY(std::ptr::null_mut());
    if RegOpenKeyExW(
        HKEY_CURRENT_USER,
        w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run"),
        None,
        KEY_READ,
        &mut key,
    )
    .is_err()
    {
        return false;
    }

    let mut len: u32 = 0;
    let result =
        RegQueryValueExW(key, w!("HideWinHide"), None, None, None, Some(&mut len)).is_ok();

    let _ = RegCloseKey(key);
    result
}

unsafe fn write_registry_entry() {
    let mut key: HKEY = HKEY(std::ptr::null_mut());
    if RegOpenKeyExW(
        HKEY_CURRENT_USER,
        w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run"),
        None,
        KEY_WRITE,
        &mut key,
    )
    .is_err()
    {
        if RegCreateKeyW(
            HKEY_CURRENT_USER,
            w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run"),
            &mut key,
        )
        .is_err()
        {
            return;
        }
    }

    let mut buf: [u16; 1024] = [0; 1024];
    let len = GetModuleFileNameW(None, &mut buf);
    if len > 0 {
        let mut quoted: Vec<u16> = Vec::with_capacity(len as usize + 3);
        quoted.push(b'"' as u16);
        quoted.extend_from_slice(&buf[..len as usize]);
        quoted.push(b'"' as u16);
        quoted.push(0);
        let path_bytes: &[u8] =
            std::slice::from_raw_parts(quoted.as_ptr() as *const u8, quoted.len() * 2);
        let _ = RegSetValueExW(
            key,
            w!("HideWinHide"),
            None,
            REG_SZ,
            Some(path_bytes),
        );
    }

    let _ = RegCloseKey(key);
}

#[allow(dead_code)]
pub unsafe fn ensure_registered() {
    if !is_enabled() {
        write_registry_entry();
    }
}

pub unsafe fn enable() {
    write_registry_entry();
}

pub unsafe fn disable() {
    let mut key: HKEY = HKEY(std::ptr::null_mut());
    if RegOpenKeyExW(
        HKEY_CURRENT_USER,
        w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run"),
        None,
        KEY_WRITE,
        &mut key,
    )
    .is_err()
    {
        return;
    }

    let _ = RegDeleteValueW(key, w!("HideWinHide"));
    let _ = RegCloseKey(key);
}
