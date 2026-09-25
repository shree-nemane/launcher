pub mod autostart_manager;
pub mod commands;
pub mod error;
pub mod execution;
pub mod models;
pub mod parser;
pub mod resolver;
pub mod storage;
pub mod validation;
pub mod window_manager;

use tauri::{Emitter, Manager};

pub fn is_two_process_mode() -> bool {
    std::env::args().any(|a| a == "--two-process") || std::env::var("LAUNCHER_TWO_PROCESS").is_ok()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    window_manager::configure_webview2_environment();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            window_manager::show_and_reset_launcher(app);
        }))
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
            if !is_two_process_mode() {
                let _ = window_manager::setup_system_tray(&app_handle);
                window_manager::setup_global_shortcut(&app_handle, &storage);
            }

            app.manage(storage);

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if is_two_process_mode() {
                    std::process::exit(0);
                }
                api.prevent_close();
                let _ = window.emit("launcher://hide", ());
                let _ = window.hide();
                if let Some(wv) = window.get_webview_window(window.label()) {
                    window_manager::suspend_webview(&wv);
                }
                window_manager::trim_process_memory();
            }
            tauri::WindowEvent::Focused(focused) => {
                let _ = window.emit("launcher://focus-changed", focused);
            }
            _ => {}
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
            commands::dialog_commands::signal_launcher_ready,
            commands::autostart_commands::is_autostart_enabled,
            commands::autostart_commands::enable_autostart,
            commands::autostart_commands::disable_autostart,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
