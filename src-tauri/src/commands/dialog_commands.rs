use tauri::{Emitter, WebviewWindow};

#[tauri::command]
pub async fn pick_folder(window: WebviewWindow) -> Result<Option<String>, String> {
    let _ = window.set_always_on_top(false);
    let folder = rfd::AsyncFileDialog::new()
        .set_title("Select Project Directory")
        .pick_folder()
        .await;
    let _ = window.set_always_on_top(true);
    let _ = window.set_focus();

    Ok(folder.map(|handle| handle.path().to_string_lossy().to_string()))
}

#[tauri::command]
pub async fn pick_executable(window: WebviewWindow) -> Result<Option<String>, String> {
    let _ = window.set_always_on_top(false);
    let file = rfd::AsyncFileDialog::new()
        .set_title("Select Application Executable")
        .add_filter("Executable Files", &["exe", "cmd", "bat", "ps1"])
        .pick_file()
        .await;
    let _ = window.set_always_on_top(true);
    let _ = window.set_focus();

    Ok(file.map(|handle| handle.path().to_string_lossy().to_string()))
}

#[tauri::command]
pub fn hide_launcher(window: WebviewWindow) -> Result<(), String> {
    if crate::is_two_process_mode() {
        std::process::exit(0);
    }
    let _ = window.emit("launcher://hide", ());
    let res = window.hide().map_err(|e| e.to_string());
    crate::window_manager::suspend_webview(&window);
    crate::window_manager::trim_process_memory();
    res
}

#[tauri::command]
pub fn close_launcher(window: WebviewWindow) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn signal_launcher_ready() {
    #[cfg(target_os = "windows")]
    unsafe {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        let event_name: Vec<u16> = OsStr::new("UniversalLauncherReadyEvent")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        extern "system" {
            fn OpenEventW(dw_desired_access: u32, b_inherit_handle: i32, lp_name: *const u16) -> *mut std::ffi::c_void;
            fn SetEvent(h_event: *mut std::ffi::c_void) -> i32;
            fn CloseHandle(h_object: *mut std::ffi::c_void) -> i32;
        }
        let h_event = OpenEventW(0x0002, 0, event_name.as_ptr());
        if !h_event.is_null() {
            SetEvent(h_event);
            CloseHandle(h_event);
        }
    }
}
