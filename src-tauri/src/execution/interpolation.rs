use crate::execution::error::PlanningError;
use crate::models::Project;

pub fn interpolate_string(template: &str, project: &Project) -> Result<String, PlanningError> {
    if !template.contains('{') {
        return Ok(template.to_string());
    }

    let mut result = template.to_string();

    if result.contains("{PROJECT_PATH}") {
        result = result.replace("{PROJECT_PATH}", &project.path);
    }

    if result.contains("{PROJECT_NAME}") {
        result = result.replace("{PROJECT_NAME}", &project.name);
    }

    if result.contains("{PROJECT_COMMAND}") {
        result = result.replace("{PROJECT_COMMAND}", &project.command);
    }

    if result.contains("{PROJECT_WORKING_DIRECTORY}") {
        let work_dir = project
            .working_directory
            .as_deref()
            .unwrap_or(&project.path);
        result = result.replace("{PROJECT_WORKING_DIRECTORY}", work_dir);
    }

    if result.contains("{PROJECT_URL}") {
        match project.url {
            Some(ref url) if !url.trim().is_empty() => {
                result = result.replace("{PROJECT_URL}", url.trim());
            }
            _ => {
                return Err(PlanningError::MissingProjectUrl {
                    project_name: project.name.clone(),
                    template: template.to_string(),
                });
            }
        }
    }

    Ok(result)
}

pub fn interpolate_arguments(
    args: &[String],
    project: &Project,
) -> Result<Vec<String>, PlanningError> {
    args.iter()
        .map(|arg| interpolate_string(arg, project))
        .collect()
}
