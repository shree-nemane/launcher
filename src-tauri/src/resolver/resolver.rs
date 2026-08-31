use crate::models::{Application, Project};
use crate::parser::{parse, ParsedCommand};
use crate::resolver::command_type::ResolvedCommand;
use crate::resolver::error::ResolverError;
use crate::storage::StorageManager;

pub fn resolve(
    parsed: &ParsedCommand,
    storage: &StorageManager,
) -> Result<ResolvedCommand, ResolverError> {
    match parsed {
        // System commands bypass storage resolution completely
        ParsedCommand::System { command } => Ok(ResolvedCommand::System {
            command: *command,
        }),

        ParsedCommand::Launch {
            project_command,
            application_commands,
            use_default_group,
        } => {
            // 1. Resolve Project (if specified)
            let resolved_project: Option<Project> = match project_command {
                Some(cmd) => {
                    let project = storage
                        .find_project_by_command(cmd)?
                        .ok_or_else(|| ResolverError::project_not_found(cmd))?;
                    Some(project)
                }
                None => None,
            };

            // 2. Resolve Applications
            let mut resolved_applications: Vec<Application> = Vec::new();
            let is_group: bool;

            if *use_default_group {
                is_group = true;
                let settings = storage.get_settings()?;
                let group_id = settings
                    .default_application_group_id
                    .ok_or_else(ResolverError::default_group_not_configured)?;

                let group = storage
                    .get_group(&group_id)
                    .map_err(|_| ResolverError::group_not_found(&group_id))?;

                // Group resolution strictly follows group.execution_order
                let app_ids = if !group.execution_order.is_empty() {
                    &group.execution_order
                } else {
                    &group.applications
                };

                for app_id in app_ids {
                    let app = storage.get_application(app_id).map_err(|_| {
                        ResolverError::corrupted_group_data(&format!(
                            "Application with ID '{}' in group '{}' not found",
                            app_id, group.id
                        ))
                    })?;
                    resolved_applications.push(app);
                }
            } else if !application_commands.is_empty() {
                is_group = false;
                for app_cmd in application_commands {
                    if let Some(app) = storage.find_application_by_command(app_cmd)? {
                        resolved_applications.push(app);
                    } else if app_cmd == "/r" || app_cmd == "/run" {
                        resolved_applications.push(Application {
                            id: "builtin_run_commands".to_string(),
                            name: "Run Project Commands".to_string(),
                            command: app_cmd.clone(),
                            executable_path: "".to_string(),
                            normal_launch: Default::default(),
                            project_launch: Some(crate::models::ProjectLaunchConfig {
                                enabled: true,
                                arguments: Vec::new(),
                            }),
                            working_directory: None,
                            icon: None,
                            created_at: "".to_string(),
                            updated_at: "".to_string(),
                        });
                    } else {
                        return Err(ResolverError::application_not_found(app_cmd));
                    }
                }
            } else {
                // Project-only command
                is_group = false;
            }

            Ok(ResolvedCommand::Launch {
                project: resolved_project,
                applications: resolved_applications,
                is_group,
            })
        }
    }
}

pub fn resolve_raw(
    input: &str,
    storage: &StorageManager,
) -> Result<ResolvedCommand, ResolverError> {
    let parsed = parse(input)?;
    resolve(&parsed, storage)
}
