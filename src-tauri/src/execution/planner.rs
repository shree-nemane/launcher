use crate::execution::command_type::{ExecutionPlan, LaunchAction};
use crate::execution::error::PlanningError;
use crate::execution::interpolation::{interpolate_arguments, interpolate_string};
use crate::models::Project;
use crate::resolver::ResolvedCommand;

#[cfg(target_os = "windows")]
fn is_windows_terminal_available() -> bool {
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            if dir.join("wt.exe").is_file() {
                return true;
            }
        }
    }
    if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
        if std::path::Path::new(&local_appdata)
            .join("Microsoft")
            .join("WindowsApps")
            .join("wt.exe")
            .is_file()
        {
            return true;
        }
    }
    false
}

fn create_run_command_actions(proj: &Project) -> Vec<LaunchAction> {
    let mut actions = Vec::new();
    let work_dir = proj
        .working_directory
        .as_deref()
        .unwrap_or(&proj.path);

    if proj.run_commands.is_empty() {
        return actions;
    }

    #[cfg(target_os = "windows")]
    {
        if is_windows_terminal_available() {
            // Bundle all project run commands into a single multi-tab Windows Terminal window
            let mut wt_args = Vec::new();
            for (idx, cmd) in proj.run_commands.iter().enumerate() {
                if idx > 0 {
                    wt_args.push(";".to_string());
                }
                let label = if cmd.name.trim().is_empty() {
                    cmd.command.clone()
                } else {
                    cmd.name.clone()
                };
                let title = format!("{}: {}", proj.name, label);
                wt_args.push("new-tab".to_string());
                wt_args.push("--title".to_string());
                wt_args.push(title);
                wt_args.push("-d".to_string());
                wt_args.push(work_dir.to_string());
                wt_args.push("cmd.exe".to_string());
                wt_args.push("/k".to_string());
                wt_args.push(cmd.command.clone());
            }

            let title = if proj.run_commands.len() == 1 {
                let label = if proj.run_commands[0].name.trim().is_empty() {
                    proj.run_commands[0].command.clone()
                } else {
                    proj.run_commands[0].name.clone()
                };
                format!("{}: {}", proj.name, label)
            } else {
                format!("{}: Dev Servers", proj.name)
            };

            actions.push(LaunchAction::Process {
                name: title,
                executable_path: "wt.exe".to_string(),
                arguments: wt_args,
                working_directory: Some(work_dir.to_string()),
            });
            return actions;
        }

        // Fallback: If wt.exe is not available, spawn individual cmd.exe windows
        for cmd in &proj.run_commands {
            let label = if cmd.name.trim().is_empty() {
                cmd.command.clone()
            } else {
                cmd.name.clone()
            };
            let title = format!("{}: {}", proj.name, label);
            actions.push(LaunchAction::Process {
                name: title,
                executable_path: "cmd.exe".to_string(),
                arguments: vec![
                    "/k".to_string(),
                    format!("cd /d \"{}\" && {}", work_dir, cmd.command),
                ],
                working_directory: Some(work_dir.to_string()),
            });
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        for cmd in &proj.run_commands {
            let label = if cmd.name.trim().is_empty() {
                cmd.command.clone()
            } else {
                cmd.name.clone()
            };
            let title = format!("{}: {}", proj.name, label);
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

fn sanitize_shell_arguments(
    executable_path: &str,
    arguments: Vec<String>,
    project_path: Option<&str>,
) -> Vec<String> {
    let lower_exe = executable_path.to_lowercase();
    let is_powershell = lower_exe.ends_with("pwsh.exe")
        || lower_exe.ends_with("powershell.exe")
        || lower_exe == "pwsh"
        || lower_exe == "powershell";
    let is_cmd = lower_exe.ends_with("cmd.exe") || lower_exe == "cmd";

    if let Some(proj_dir) = project_path {
        if is_powershell {
            // Remove bare project path argument that causes PowerShell to execute the directory as a script
            let mut cleaned: Vec<String> = arguments
                .into_iter()
                .filter(|arg| {
                    let trimmed = arg.trim();
                    trimmed != proj_dir && trimmed != format!("\"{}\"", proj_dir)
                })
                .collect();

            // Ensure -NoExit is present so PowerShell stays open
            if !cleaned
                .iter()
                .any(|a| a.eq_ignore_ascii_case("-NoExit") || a.eq_ignore_ascii_case("--NoExit"))
            {
                cleaned.insert(0, "-NoExit".to_string());
            }
            return cleaned;
        } else if is_cmd {
            let mut cleaned: Vec<String> = arguments
                .into_iter()
                .filter(|arg| {
                    let trimmed = arg.trim();
                    trimmed != proj_dir && trimmed != format!("\"{}\"", proj_dir)
                })
                .collect();
            if !cleaned.iter().any(|a| a.eq_ignore_ascii_case("/k")) {
                cleaned.insert(0, "/k".to_string());
            }
            return cleaned;
        }
    }

    arguments
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
                                if is_windows_terminal_available() {
                                    actions.push(LaunchAction::Process {
                                        name: format!("{}: Terminal", proj.name),
                                        executable_path: "wt.exe".to_string(),
                                        arguments: vec![
                                            "-d".to_string(),
                                            work_dir.to_string(),
                                        ],
                                        working_directory: Some(work_dir.to_string()),
                                    });
                                } else {
                                    actions.push(LaunchAction::Process {
                                        name: format!("{}: Command Prompt", proj.name),
                                        executable_path: "cmd.exe".to_string(),
                                        arguments: vec![
                                            "/k".to_string(),
                                            format!("cd /d \"{}\"", work_dir),
                                        ],
                                        working_directory: Some(work_dir.to_string()),
                                    });
                                }
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
                    let raw_args = interpolate_arguments(&pl_config.arguments, proj)?;
                    let args = sanitize_shell_arguments(
                        &app.executable_path,
                        raw_args,
                        Some(&proj.path),
                    );

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
                    let raw_args = app.normal_launch.arguments.clone();
                    let args = sanitize_shell_arguments(
                        &app.executable_path,
                        raw_args,
                        project.as_ref().map(|p| p.path.as_str()),
                    );
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
