use tauri::WebviewWindow;

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
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn close_launcher(window: WebviewWindow) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}
