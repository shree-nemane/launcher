// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
fn delegate_to_host_if_needed() -> bool {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let args: Vec<String> = std::env::args().collect();
    let is_two_process = args.iter().any(|a| a == "--two-process") || std::env::var("LAUNCHER_TWO_PROCESS").is_ok();
    let is_explicit_fallback = args.iter().any(|a| a == "--fallback" || a == "--single-process");

    if is_two_process || is_explicit_fallback {
        return false;
    }

    extern "system" {
        fn FindWindowW(lp_class_name: *const u16, lp_window_name: *const u16) -> *mut std::ffi::c_void;
        fn PostMessageW(h_wnd: *mut std::ffi::c_void, msg: u32, w_param: usize, l_param: isize) -> i32;
    }

    let class_name: Vec<u16> = OsStr::new("UniversalLauncherHostClass")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let window_title: Vec<u16> = OsStr::new("UniversalLauncherHost")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    // 1. Check if launcher-host.exe is already running
    let existing_host_hwnd = unsafe { FindWindowW(class_name.as_ptr(), window_title.as_ptr()) };
    if !existing_host_hwnd.is_null() {
        // YES: Signal host with IDM_OPEN (2001) and exit immediately
        const WM_COMMAND: u32 = 0x0111;
        const IDM_OPEN: usize = 2001;
        unsafe {
            PostMessageW(existing_host_hwnd, WM_COMMAND, IDM_OPEN, 0);
        }
        return true;
    }

    // 2. NO: Check if launcher-host.exe is available to start
    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(dir) = current_exe.parent() {
            let host_exe = dir.join("launcher-host.exe");
            if host_exe.exists() {
                let _ = std::process::Command::new(&host_exe)
                    .arg("--open")
                    .spawn();
                return true;
            }
        }
    }

    false
}

fn main() {
    #[cfg(target_os = "windows")]
    {
        if delegate_to_host_if_needed() {
            return;
        }
    }

    launcher_lib::window_manager::configure_webview2_environment();
    launcher_lib::run()
}
