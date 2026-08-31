use crate::error::AppError;
use crate::models::{Application, ApplicationGroup, Project, Settings};
use crate::storage::StorageManager;
use tauri::State;

// ==========================================
// PROJECT COMMANDS
// ==========================================

#[tauri::command]
pub fn get_projects(storage: State<'_, StorageManager>) -> Result<Vec<Project>, AppError> {
    storage.get_projects()
}

#[tauri::command]
pub fn get_project(id: String, storage: State<'_, StorageManager>) -> Result<Project, AppError> {
    storage.get_project(&id)
}

#[tauri::command]
pub fn create_project(project: Project, storage: State<'_, StorageManager>) -> Result<Project, AppError> {
    storage.create_project(project)
}

#[tauri::command]
pub fn update_project(project: Project, storage: State<'_, StorageManager>) -> Result<Project, AppError> {
    storage.update_project(project)
}

#[tauri::command]
pub fn delete_project(id: String, storage: State<'_, StorageManager>) -> Result<(), AppError> {
    storage.delete_project(&id)
}

// ==========================================
// APPLICATION COMMANDS
// ==========================================

#[tauri::command]
pub fn get_applications(storage: State<'_, StorageManager>) -> Result<Vec<Application>, AppError> {
    storage.get_applications()
}

#[tauri::command]
pub fn get_application(id: String, storage: State<'_, StorageManager>) -> Result<Application, AppError> {
    storage.get_application(&id)
}

#[tauri::command]
pub fn create_application(
    application: Application,
    storage: State<'_, StorageManager>,
) -> Result<Application, AppError> {
    storage.create_application(application)
}

#[tauri::command]
pub fn update_application(
    application: Application,
    storage: State<'_, StorageManager>,
) -> Result<Application, AppError> {
    storage.update_application(application)
}

#[tauri::command]
pub fn delete_application(id: String, storage: State<'_, StorageManager>) -> Result<(), AppError> {
    storage.delete_application(&id)
}

// ==========================================
// GROUP COMMANDS
// ==========================================

#[tauri::command]
pub fn get_groups(storage: State<'_, StorageManager>) -> Result<Vec<ApplicationGroup>, AppError> {
    storage.get_groups()
}

#[tauri::command]
pub fn get_group(id: String, storage: State<'_, StorageManager>) -> Result<ApplicationGroup, AppError> {
    storage.get_group(&id)
}

#[tauri::command]
pub fn create_group(group: ApplicationGroup, storage: State<'_, StorageManager>) -> Result<ApplicationGroup, AppError> {
    storage.create_group(group)
}

#[tauri::command]
pub fn update_group(group: ApplicationGroup, storage: State<'_, StorageManager>) -> Result<ApplicationGroup, AppError> {
    storage.update_group(group)
}

#[tauri::command]
pub fn delete_group(id: String, storage: State<'_, StorageManager>) -> Result<(), AppError> {
    storage.delete_group(&id)
}

// ==========================================
// SETTINGS COMMANDS
// ==========================================

#[tauri::command]
pub fn get_settings(storage: State<'_, StorageManager>) -> Result<Settings, AppError> {
    storage.get_settings()
}

#[tauri::command]
pub fn update_settings(
    app: tauri::AppHandle,
    settings: Settings,
    storage: State<'_, StorageManager>,
) -> Result<Settings, AppError> {
    let old_settings = storage.get_settings()?;
    let updated = storage.update_settings(settings)?;

    if old_settings.global_shortcut != updated.global_shortcut {
        crate::window_manager::register_global_shortcut_with_fallback(&app, &updated.global_shortcut);
    }

    Ok(updated)
}
