use crate::error::AppError;
use crate::models::Project;
use std::path::Path;

const RESERVED_COMMANDS: &[&str] = &[
    "add-project",
    "add-app",
    "manage-projects",
    "manage-apps",
    "//",
];

pub fn validate_project(
    project: &Project,
    existing_projects: &[Project],
    is_update: bool,
) -> Result<(), AppError> {
    let name_trimmed = project.name.trim();
    if name_trimmed.is_empty() {
        return Err(AppError::Validation("Project name cannot be empty".to_string()));
    }

    let cmd_trimmed = project.command.trim();
    if cmd_trimmed.is_empty() {
        return Err(AppError::Validation("Project command cannot be empty".to_string()));
    }

    if cmd_trimmed.starts_with('/') {
        return Err(AppError::Validation(
            "Project command cannot start with '/'".to_string(),
        ));
    }

    if cmd_trimmed.contains('/') {
        return Err(AppError::Validation(
            "Project command cannot contain '/'".to_string(),
        ));
    }

    let cmd_lower = cmd_trimmed.to_lowercase();
    for reserved in RESERVED_COMMANDS {
        if cmd_lower == reserved.to_lowercase() {
            return Err(AppError::Validation(format!(
                "Command '{}' is a reserved system command",
                cmd_trimmed
            )));
        }
    }

    for existing in existing_projects {
        if is_update && existing.id == project.id {
            continue;
        }
        if existing.command.trim().eq_ignore_ascii_case(cmd_trimmed) {
            return Err(AppError::Validation(format!(
                "Project command '{}' already exists",
                cmd_trimmed
            )));
        }
    }

    let path = Path::new(&project.path);
    if !path.exists() {
        return Err(AppError::Validation(format!(
            "Project path does not exist: {}",
            project.path
        )));
    }
    if !path.is_dir() {
        return Err(AppError::Validation(format!(
            "Project path is not a directory: {}",
            project.path
        )));
    }

    if let Some(ref work_dir) = project.working_directory {
        let work_path = Path::new(work_dir);
        if !work_path.exists() || !work_path.is_dir() {
            return Err(AppError::Validation(format!(
                "Working directory is invalid or does not exist: {}",
                work_dir
            )));
        }
    }

    Ok(())
}
