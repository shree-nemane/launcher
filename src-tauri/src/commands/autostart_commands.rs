#[tauri::command]
pub fn is_autostart_enabled(app: tauri::AppHandle) -> Result<bool, String> {
    if crate::is_two_process_mode() {
        crate::autostart_manager::is_host_autostart_enabled()
    } else {
        use tauri_plugin_autostart::ManagerExt;
        app.autolaunch().is_enabled().map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn enable_autostart(app: tauri::AppHandle) -> Result<(), String> {
    if crate::is_two_process_mode() {
        crate::autostart_manager::enable_host_autostart()
    } else {
        use tauri_plugin_autostart::ManagerExt;
        app.autolaunch().enable().map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn disable_autostart(app: tauri::AppHandle) -> Result<(), String> {
    if crate::is_two_process_mode() {
        crate::autostart_manager::disable_host_autostart()
    } else {
        use tauri_plugin_autostart::ManagerExt;
        app.autolaunch().disable().map_err(|e| e.to_string())
    }
}
