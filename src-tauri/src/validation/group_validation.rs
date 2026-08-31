use crate::error::AppError;
use crate::models::{Application, ApplicationGroup};
use std::collections::HashSet;

pub fn validate_group(
    group: &ApplicationGroup,
    existing_applications: &[Application],
) -> Result<(), AppError> {
    let name_trimmed = group.name.trim();
    if name_trimmed.is_empty() {
        return Err(AppError::Validation(
            "Application group name cannot be empty".to_string(),
        ));
    }

    // Check for duplicate applications in the group
    let mut seen_apps = HashSet::new();
    for app_id in &group.applications {
        if !seen_apps.insert(app_id) {
            return Err(AppError::Validation(format!(
                "Duplicate application ID '{}' found in application group",
                app_id
            )));
        }

        if !existing_applications.iter().any(|a| a.id == *app_id) {
            return Err(AppError::Validation(format!(
                "Referenced application ID '{}' does not exist",
                app_id
            )));
        }
    }

    // Check for duplicate applications in execution order
    let mut seen_order = HashSet::new();
    for app_id in &group.execution_order {
        if !seen_order.insert(app_id) {
            return Err(AppError::Validation(format!(
                "Duplicate application ID '{}' found in group execution order",
                app_id
            )));
        }

        if !group.applications.contains(app_id) {
            return Err(AppError::Validation(format!(
                "Execution order contains application ID '{}' which is not in the group's application list",
                app_id
            )));
        }
    }

    Ok(())
}
