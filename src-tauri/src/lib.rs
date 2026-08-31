pub mod commands;
pub mod error;
pub mod execution;
pub mod models;
pub mod parser;
pub mod resolver;
pub mod storage;
pub mod validation;
pub mod window_manager;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data dir");
            let storage = storage::StorageManager::new(app_data_dir)
                .expect("failed to initialize storage manager");

            let app_handle = app.handle().clone();
            let _ = window_manager::setup_system_tray(&app_handle);
            window_manager::setup_global_shortcut(&app_handle, &storage);

            app.manage(storage);

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::storage_commands::get_projects,
            commands::storage_commands::get_project,
            commands::storage_commands::create_project,
            commands::storage_commands::update_project,
            commands::storage_commands::delete_project,
            commands::storage_commands::get_applications,
            commands::storage_commands::get_application,
            commands::storage_commands::create_application,
            commands::storage_commands::update_application,
            commands::storage_commands::delete_application,
            commands::storage_commands::get_groups,
            commands::storage_commands::get_group,
            commands::storage_commands::create_group,
            commands::storage_commands::update_group,
            commands::storage_commands::delete_group,
            commands::storage_commands::get_settings,
            commands::storage_commands::update_settings,
            commands::parser_commands::parse_command,
            commands::resolver_commands::resolve_command,
            commands::execution_commands::plan_command,
            commands::execution_commands::execute_command,
            commands::dialog_commands::pick_folder,
            commands::dialog_commands::pick_executable,
            commands::dialog_commands::hide_launcher,
            commands::dialog_commands::close_launcher,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
