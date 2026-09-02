// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    launcher_lib::window_manager::configure_webview2_environment();
    launcher_lib::run()
}
