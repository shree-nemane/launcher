use std::path::PathBuf;

const REG_KEY_RUN: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const REG_VALUE_NAME: &str = "UniversalLauncher";
const REG_VALUE_LEGACY: &str = "Universal launcher";

/// Locates the absolute path to launcher-host.exe.
/// Primary location: same directory as universal-launcher.exe.
/// Fallback: %LOCALAPPDATA%\Programs\UniversalLauncher\launcher-host.exe.
pub fn get_host_executable_path() -> Result<PathBuf, String> {
    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(dir) = current_exe.parent() {
            // 1. Check same directory (production install & normal runtime)
            let candidate = dir.join("launcher-host.exe");
            if candidate.exists() {
                return Ok(candidate);
            }
            // 2. Check parent directory (e.g. when running under target/debug/deps or target/release/deps)
            if let Some(parent_dir) = dir.parent() {
                let candidate2 = parent_dir.join("launcher-host.exe");
                if candidate2.exists() {
                    return Ok(candidate2);
                }
                let release_candidate = parent_dir.join("release").join("launcher-host.exe");
                if release_candidate.exists() {
                    return Ok(release_candidate);
                }
            }
        }
    }

    if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
        let fallback = PathBuf::from(local_appdata)
            .join("Programs")
            .join("UniversalLauncher")
            .join("launcher-host.exe");
        if fallback.exists() {
            return Ok(fallback);
        }
    }

    Err("Could not locate launcher-host.exe on disk".to_string())
}

#[cfg(target_os = "windows")]
mod win32 {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    const HKEY_CURRENT_USER: isize = 0x80000001u32 as i32 as isize;
    const KEY_READ: u32 = 0x20019;
    const KEY_WRITE: u32 = 0x20006;
    const REG_SZ: u32 = 1;
    const ERROR_SUCCESS: i32 = 0;
    const ERROR_FILE_NOT_FOUND: i32 = 2;

    #[link(name = "advapi32")]
    extern "system" {
        fn RegOpenKeyExW(
            hKey: isize,
            lpSubKey: *const u16,
            ulOptions: u32,
            samDesired: u32,
            phkResult: *mut isize,
        ) -> i32;

        fn RegQueryValueExW(
            hKey: isize,
            lpValueName: *const u16,
            lpReserved: *mut u32,
            lpType: *mut u32,
            lpData: *mut u8,
            lpcbData: *mut u32,
        ) -> i32;

        fn RegSetValueExW(
            hKey: isize,
            lpValueName: *const u16,
            Reserved: u32,
            dwType: u32,
            lpData: *const u8,
            cbData: u32,
        ) -> i32;

        fn RegDeleteValueW(hKey: isize, lpValueName: *const u16) -> i32;

        fn RegCloseKey(hKey: isize) -> i32;
    }

    fn to_wide(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
    }

    pub fn query_run_entry(value_name: &str) -> Result<Option<String>, String> {
        let subkey = to_wide(super::REG_KEY_RUN);
        let val_name = to_wide(value_name);
        let mut h_key: isize = 0;

        unsafe {
            let res = RegOpenKeyExW(HKEY_CURRENT_USER, subkey.as_ptr(), 0, KEY_READ, &mut h_key);
            if res != ERROR_SUCCESS {
                return Err(format!("RegOpenKeyExW failed with code {}", res));
            }

            let mut data_type: u32 = 0;
            let mut byte_count: u32 = 0;

            // Probe buffer size
            let res = RegQueryValueExW(
                h_key,
                val_name.as_ptr(),
                std::ptr::null_mut(),
                &mut data_type,
                std::ptr::null_mut(),
                &mut byte_count,
            );

            if res == ERROR_FILE_NOT_FOUND {
                RegCloseKey(h_key);
                return Ok(None);
            }

            if res != ERROR_SUCCESS {
                RegCloseKey(h_key);
                return Err(format!("RegQueryValueExW probe failed with code {}", res));
            }

            if data_type != REG_SZ && data_type != 2 /* REG_EXPAND_SZ */ {
                RegCloseKey(h_key);
                return Ok(None);
            }

            let mut buffer: Vec<u8> = vec![0u8; byte_count as usize];
            let res = RegQueryValueExW(
                h_key,
                val_name.as_ptr(),
                std::ptr::null_mut(),
                &mut data_type,
                buffer.as_mut_ptr(),
                &mut byte_count,
            );

            RegCloseKey(h_key);

            if res != ERROR_SUCCESS {
                return Err(format!("RegQueryValueExW read failed with code {}", res));
            }

            // Convert UTF-16LE bytes to Rust String
            let u16_slice: &[u16] = std::slice::from_raw_parts(
                buffer.as_ptr() as *const u16,
                buffer.len() / 2,
            );
            let trimmed = match u16_slice.iter().position(|&c| c == 0) {
                Some(len) => &u16_slice[..len],
                None => u16_slice,
            };
            let s = String::from_utf16_lossy(trimmed);
            Ok(Some(s))
        }
    }

    pub fn set_run_entry(value_name: &str, cmd_value: &str) -> Result<(), String> {
        let subkey = to_wide(super::REG_KEY_RUN);
        let val_name = to_wide(value_name);
        let val_data = to_wide(cmd_value);
        let mut h_key: isize = 0;

        unsafe {
            let res = RegOpenKeyExW(HKEY_CURRENT_USER, subkey.as_ptr(), 0, KEY_WRITE, &mut h_key);
            if res != ERROR_SUCCESS {
                return Err(format!("RegOpenKeyExW write failed with code {}", res));
            }

            let byte_count = (val_data.len() * std::mem::size_of::<u16>()) as u32;
            let res = RegSetValueExW(
                h_key,
                val_name.as_ptr(),
                0,
                REG_SZ,
                val_data.as_ptr() as *const u8,
                byte_count,
            );

            RegCloseKey(h_key);

            if res != ERROR_SUCCESS {
                return Err(format!("RegSetValueExW failed with code {}", res));
            }

            Ok(())
        }
    }

    pub fn delete_run_entry(value_name: &str) -> Result<(), String> {
        let subkey = to_wide(super::REG_KEY_RUN);
        let val_name = to_wide(value_name);
        let mut h_key: isize = 0;

        unsafe {
            let res = RegOpenKeyExW(HKEY_CURRENT_USER, subkey.as_ptr(), 0, KEY_WRITE, &mut h_key);
            if res != ERROR_SUCCESS {
                return Err(format!("RegOpenKeyExW delete failed with code {}", res));
            }

            let res = RegDeleteValueW(h_key, val_name.as_ptr());
            RegCloseKey(h_key);

            if res != ERROR_SUCCESS && res != ERROR_FILE_NOT_FOUND {
                return Err(format!("RegDeleteValueW failed with code {}", res));
            }

            Ok(())
        }
    }
}

pub fn is_host_autostart_enabled() -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        // Check primary value name
        if let Some(entry) = win32::query_run_entry(REG_VALUE_NAME)? {
            if entry.to_lowercase().contains("launcher-host.exe") {
                return Ok(true);
            }
        }
        // Also check legacy/product value name if present
        if let Some(entry) = win32::query_run_entry(REG_VALUE_LEGACY)? {
            if entry.to_lowercase().contains("launcher-host.exe") {
                return Ok(true);
            }
        }
        Ok(false)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(false)
    }
}

pub fn enable_host_autostart() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let host_path = get_host_executable_path()?;
        let cmd = format!("\"{}\"", host_path.to_string_lossy());

        // Remove any previous single-process entry to prevent duplicate startup
        let _ = win32::delete_run_entry(REG_VALUE_LEGACY);

        // Register launcher-host.exe under HKCU\Software\Microsoft\Windows\CurrentVersion\Run
        win32::set_run_entry(REG_VALUE_NAME, &cmd)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(())
    }
}

pub fn disable_host_autostart() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // Delete both value names to guarantee idempotent cleanup
        let _ = win32::delete_run_entry(REG_VALUE_LEGACY);
        win32::delete_run_entry(REG_VALUE_NAME)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(())
    }
}
