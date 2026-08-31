use crate::execution::command_type::{ExecutionPlan, LaunchAction};
use crate::execution::error::PlanningError;
use crate::execution::interpolation::{interpolate_arguments, interpolate_string};
use crate::models::Project;
use crate::resolver::ResolvedCommand;

#[cfg(target_os = "windows")]
fn detect_powershell_executable() -> &'static str {
    // Check if pwsh.exe (PowerShell 7) is in PATH
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            if dir.join("pwsh.exe").is_file() {
                return "pwsh.exe";
            }
        }
    }
    // Check standard Program Files installation paths for PowerShell 7
    if std::path::Path::new("C:\\Program Files\\PowerShell\\7\\pwsh.exe").is_file() {
        return "C:\\Program Files\\PowerShell\\7\\pwsh.exe";
    }
    if std::path::Path::new("C:\\Program Files\\PowerShell\\7-preview\\pwsh.exe").is_file() {
        return "C:\\Program Files\\PowerShell\\7-preview\\pwsh.exe";
    }
    // Fallback to built-in Windows PowerShell if PowerShell 7 is not installed
    "powershell.exe"
}

fn create_run_command_actions(proj: &Project) -> Vec<LaunchAction> {
    let mut actions = Vec::new();
    let work_dir = proj
        .working_directory
        .as_deref()
        .unwrap_or(&proj.path);

    for cmd in &proj.run_commands {
        let label = if cmd.name.trim().is_empty() {
            cmd.command.clone()
        } else {
            cmd.name.clone()
        };
        let title = format!("{}: {}", proj.name, label);

        #[cfg(target_os = "windows")]
        {
            let ps_exe = detect_powershell_executable();
            actions.push(LaunchAction::Process {
                name: title.clone(),
                executable_path: "cmd.exe".to_string(),
                arguments: vec![
                    "/c".to_string(),
                    "start".to_string(),
                    title,
                    ps_exe.to_string(),
                    "-NoExit".to_string(),
                    "-Command".to_string(),
                    format!("Set-Location '{}'; {}", work_dir, cmd.command),
                ],
                working_directory: Some(work_dir.to_string()),
            });
        }

        #[cfg(not(target_os = "windows"))]
        {
            actions.push(LaunchAction::Process {
                name: title,
                executable_path: "sh".to_string(),
                arguments: vec!["-c".to_string(), cmd.command.clone()],
                working_directory: Some(work_dir.to_string()),
            });
        }
    }

    actions
}

pub fn plan(resolved: &ResolvedCommand) -> Result<ExecutionPlan, PlanningError> {
    match resolved {
        ResolvedCommand::System { command } => Ok(ExecutionPlan::System {
            command: *command,
        }),

        ResolvedCommand::Launch {
            project,
            applications,
            is_group: _,
        } => {
            let mut actions: Vec<LaunchAction> = Vec::new();

            if let Some(ref proj) = project {
                if applications.is_empty() {
                    // Project-only launch -> Always open project folder in File Explorer
                    actions.push(LaunchAction::OpenFolder {
                        name: proj.name.clone(),
                        path: proj.path.clone(),
                    });
                    return Ok(ExecutionPlan::Launch { actions });
                }
            }

            // Launch applications
            for app in applications {
                if app.id == "builtin_run_commands" {
                    if let Some(ref proj) = project {
                        if !proj.run_commands.is_empty() {
                            actions.extend(create_run_command_actions(proj));
                        } else {
                            // If project has no run commands configured, open terminal in project directory
                            let work_dir = proj
                                .working_directory
                                .as_deref()
                                .unwrap_or(&proj.path);
                            #[cfg(target_os = "windows")]
                            {
                                let ps_exe = detect_powershell_executable();
                                actions.push(LaunchAction::Process {
                                    name: format!("{}: PowerShell", proj.name),
                                    executable_path: "cmd.exe".to_string(),
                                    arguments: vec![
                                        "/c".to_string(),
                                        "start".to_string(),
                                        format!("{}: PowerShell", proj.name),
                                        ps_exe.to_string(),
                                        "-NoExit".to_string(),
                                        "-Command".to_string(),
                                        format!("Set-Location '{}'", work_dir),
                                    ],
                                    working_directory: Some(work_dir.to_string()),
                                });
                            }
                            #[cfg(not(target_os = "windows"))]
                            {
                                actions.push(LaunchAction::OpenFolder {
                                    name: proj.name.clone(),
                                    path: proj.path.clone(),
                                });
                            }
                        }
                    }
                    continue;
                }

                let is_project_launch = project.is_some()
                    && app
                        .project_launch
                        .as_ref()
                        .map_or(false, |pl| pl.enabled);

                let (arguments, working_directory) = if is_project_launch {
                    let proj = project.as_ref().unwrap();
                    let pl_config = app.project_launch.as_ref().unwrap();
                    let args = interpolate_arguments(&pl_config.arguments, proj)?;

                    let work_dir = if let Some(ref wd) = app.working_directory {
                        Some(interpolate_string(wd, proj)?)
                    } else {
                        let fallback = proj
                            .working_directory
                            .as_deref()
                            .unwrap_or(&proj.path);
                        Some(fallback.to_string())
                    };

                    (args, work_dir)
                } else {
                    // Normal application launch
                    let args = app.normal_launch.arguments.clone();
                    let work_dir = app.working_directory.clone();
                    (args, work_dir)
                };

                actions.push(LaunchAction::Process {
                    name: app.name.clone(),
                    executable_path: app.executable_path.clone(),
                    arguments,
                    working_directory,
                });
            }

            Ok(ExecutionPlan::Launch { actions })
        }
    }
}
