use crate::error::AppError;
use crate::models::Application;
use std::path::Path;

pub fn validate_application(
    application: &Application,
    existing_applications: &[Application],
    is_update: bool,
) -> Result<(), AppError> {
    let name_trimmed = application.name.trim();
    if name_trimmed.is_empty() {
        return Err(AppError::Validation(
            "Application name cannot be empty".to_string(),
        ));
    }

    let cmd_trimmed = application.command.trim();
    if cmd_trimmed.is_empty() {
        return Err(AppError::Validation(
            "Application command cannot be empty".to_string(),
        ));
    }

    if !cmd_trimmed.starts_with('/') {
        return Err(AppError::Validation(
            "Application command must start with '/'".to_string(),
        ));
    }

    if cmd_trimmed == "//" {
        return Err(AppError::Validation(
            "Application command cannot be '//' (reserved for default application group)".to_string(),
        ));
    }

    if cmd_trimmed.contains(char::is_whitespace) {
        return Err(AppError::Validation(
            "Application command cannot contain spaces".to_string(),
        ));
    }

    for existing in existing_applications {
        if is_update && existing.id == application.id {
            continue;
        }
        if existing.command.trim().eq_ignore_ascii_case(cmd_trimmed) {
            return Err(AppError::Validation(format!(
                "Application command '{}' already exists",
                cmd_trimmed
            )));
        }
    }

    let exe_path = Path::new(&application.executable_path);
    let has_separators = application.executable_path.contains('/') || application.executable_path.contains('\\');
    if (exe_path.is_absolute() || has_separators) && !exe_path.exists() {
        return Err(AppError::Validation(format!(
            "Executable path does not exist: {}",
            application.executable_path
        )));
    }
    if (exe_path.is_absolute() || has_separators) && !exe_path.is_file() {
        return Err(AppError::Validation(format!(
            "Executable path is not a file: {}",
            application.executable_path
        )));
    }

    if let Some(ref work_dir) = application.working_directory {
        if work_dir != "{PROJECT_PATH}" {
            let work_path = Path::new(work_dir);
            if !work_path.exists() || !work_path.is_dir() {
                return Err(AppError::Validation(format!(
                    "Working directory is invalid or does not exist: {}",
                    work_dir
                )));
            }
        }
    }

    Ok(())
}
