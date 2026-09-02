use crate::execution::command_type::{ExecutionActionResult, ExecutionPlan, ExecutionResult, LaunchAction};
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub enum ExecutionFailureReason {
    ExecutableNotFound(String),
    ExecutableNotAFile(String),
    FolderNotFound(String),
    FolderNotADirectory(String),
    WorkingDirectoryNotFound(String),
    WorkingDirectoryNotADirectory(String),
    PermissionDenied(String),
    IoError(String),
}

impl ExecutionFailureReason {
    pub fn to_user_message(&self, name: &str) -> String {
        match self {
            Self::ExecutableNotFound(path) => {
                format!(
                    "Executable for '{}' was not found at '{}'. It may have been moved, renamed, or uninstalled.",
                    name, path
                )
            }
            Self::ExecutableNotAFile(path) => {
                format!(
                    "Executable path for '{}' is a directory, not an executable file: '{}'.",
                    name, path
                )
            }
            Self::FolderNotFound(path) => {
                format!(
                    "Project folder for '{}' was not found at '{}'. Check that the folder or drive is available.",
                    name, path
                )
            }
            Self::FolderNotADirectory(path) => {
                format!(
                    "Project path for '{}' is a file, not a directory: '{}'.",
                    name, path
                )
            }
            Self::WorkingDirectoryNotFound(path) => {
                format!("Working directory for '{}' was not found at '{}'.", name, path)
            }
            Self::WorkingDirectoryNotADirectory(path) => {
                format!(
                    "Working directory for '{}' is a file, not a directory: '{}'.",
                    name, path
                )
            }
            Self::PermissionDenied(path) => {
                format!(
                    "Permission denied when launching '{}' at '{}'. It may require elevated administrator privileges.",
                    name, path
                )
            }
            Self::IoError(details) => {
                format!("Failed to launch '{}': {}", name, details)
            }
        }
    }
}

pub fn execute(plan: &ExecutionPlan) -> ExecutionResult {
    match plan {
        ExecutionPlan::System { command } => ExecutionResult {
            success: true,
            action_results: Vec::new(),
            system_command: Some(*command),
        },

        ExecutionPlan::Launch { actions } => {
            let mut action_results = Vec::new();

            for (index, action) in actions.iter().enumerate() {
                let result = match action {
                    LaunchAction::OpenFolder { name, path } => {
                        let path_obj = Path::new(path);

                        // Execution-time preflight path validation
                        if !path_obj.exists() {
                            let failure = ExecutionFailureReason::FolderNotFound(path.clone());
                            ExecutionActionResult {
                                action_index: index,
                                name: name.clone(),
                                success: false,
                                error: Some(failure.to_user_message(name)),
                            }
                        } else if !path_obj.is_dir() {
                            let failure = ExecutionFailureReason::FolderNotADirectory(path.clone());
                            ExecutionActionResult {
                                action_index: index,
                                name: name.clone(),
                                success: false,
                                error: Some(failure.to_user_message(name)),
                            }
                        } else {
                            match open_folder_detached(path) {
                                Ok(_) => ExecutionActionResult {
                                    action_index: index,
                                    name: name.clone(),
                                    success: true,
                                    error: None,
                                },
                                Err(e) => {
                                    let failure = if e.kind() == ErrorKind::PermissionDenied {
                                        ExecutionFailureReason::PermissionDenied(path.clone())
                                    } else {
                                        ExecutionFailureReason::IoError(e.to_string())
                                    };
                                    ExecutionActionResult {
                                        action_index: index,
                                        name: name.clone(),
                                        success: false,
                                        error: Some(failure.to_user_message(name)),
                                    }
                                }
                            }
                        }
                    }

                    LaunchAction::Process {
                        name,
                        executable_path,
                        arguments,
                        working_directory,
                    } => {
                        let exe_obj = Path::new(executable_path);

                        // If path is absolute or has directory components, validate file existence
                        if (exe_obj.is_absolute()
                            || executable_path.contains('/')
                            || executable_path.contains('\\'))
                            && !exe_obj.exists()
                        {
                            let failure = ExecutionFailureReason::ExecutableNotFound(
                                executable_path.clone(),
                            );
                            ExecutionActionResult {
                                action_index: index,
                                name: name.clone(),
                                success: false,
                                error: Some(failure.to_user_message(name)),
                            }
                        } else if (exe_obj.is_absolute()
                            || executable_path.contains('/')
                            || executable_path.contains('\\'))
                            && !exe_obj.is_file()
                        {
                            let failure = ExecutionFailureReason::ExecutableNotAFile(
                                executable_path.clone(),
                            );
                            ExecutionActionResult {
                                action_index: index,
                                name: name.clone(),
                                success: false,
                                error: Some(failure.to_user_message(name)),
                            }
                        } else if let Some(ref wd) = working_directory {
                            let wd_obj = Path::new(wd);
                            if !wd_obj.exists() {
                                let failure =
                                    ExecutionFailureReason::WorkingDirectoryNotFound(wd.clone());
                                ExecutionActionResult {
                                    action_index: index,
                                    name: name.clone(),
                                    success: false,
                                    error: Some(failure.to_user_message(name)),
                                }
                            } else if !wd_obj.is_dir() {
                                let failure = ExecutionFailureReason::WorkingDirectoryNotADirectory(
                                    wd.clone(),
                                );
                                ExecutionActionResult {
                                    action_index: index,
                                    name: name.clone(),
                                    success: false,
                                    error: Some(failure.to_user_message(name)),
                                }
                            } else {
                                spawn_process(index, name, executable_path, arguments, working_directory)
                            }
                        } else {
                            spawn_process(index, name, executable_path, arguments, working_directory)
                        }
                    }
                };

                action_results.push(result);
            }

            let success = action_results.iter().all(|r| r.success);

            ExecutionResult {
                success,
                action_results,
                system_command: None,
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn resolve_executable_and_args(executable_path: &str, arguments: &[String]) -> (String, Vec<String>) {
    let lower = executable_path.to_lowercase();

    // 1. If it's a batch script (.cmd or .bat), launch via cmd.exe /c
    if lower.ends_with(".cmd") || lower.ends_with(".bat") {
        let mut cmd_args = vec!["/c".to_string(), executable_path.to_string()];
        cmd_args.extend_from_slice(arguments);
        return ("cmd.exe".to_string(), cmd_args);
    }

    // 2. If it is "code" or "code.exe" and not an absolute path, resolve actual Code.exe
    if lower == "code" || lower == "code.exe" {
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let user_code = Path::new(&local_app_data)
                .join("Programs")
                .join("Microsoft VS Code")
                .join("Code.exe");
            if user_code.is_file() {
                return (user_code.to_string_lossy().to_string(), arguments.to_vec());
            }
        }
        let sys_code = Path::new("C:\\Program Files\\Microsoft VS Code\\Code.exe");
        if sys_code.is_file() {
            return (sys_code.to_string_lossy().to_string(), arguments.to_vec());
        }
        let sys_code_x86 = Path::new("C:\\Program Files (x86)\\Microsoft VS Code\\Code.exe");
        if sys_code_x86.is_file() {
            return (sys_code_x86.to_string_lossy().to_string(), arguments.to_vec());
        }
    }

    (executable_path.to_string(), arguments.to_vec())
}

fn spawn_process(
    action_index: usize,
    name: &str,
    executable_path: &str,
    arguments: &[String],
    working_directory: &Option<String>,
) -> ExecutionActionResult {
    #[cfg(target_os = "windows")]
    let (real_exe, real_args) = resolve_executable_and_args(executable_path, arguments);

    #[cfg(not(target_os = "windows"))]
    let (real_exe, real_args) = (executable_path.to_string(), arguments.to_vec());

    let mut cmd = Command::new(&real_exe);
    cmd.args(&real_args);

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NEW_CONSOLE: u32 = 0x00000010;
        cmd.creation_flags(CREATE_NEW_CONSOLE);
    }

    if let Some(ref wd) = working_directory {
        cmd.current_dir(wd);
    }

    match cmd.spawn() {
        Ok(_) => ExecutionActionResult {
            action_index,
            name: name.to_string(),
            success: true,
            error: None,
        },
        Err(e) => {
            let failure = if e.kind() == ErrorKind::PermissionDenied {
                ExecutionFailureReason::PermissionDenied(executable_path.to_string())
            } else {
                ExecutionFailureReason::IoError(e.to_string())
            };

            ExecutionActionResult {
                action_index,
                name: name.to_string(),
                success: false,
                error: Some(failure.to_user_message(name)),
            }
        }
    }
}

fn open_folder_detached(path: &str) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer").arg(path).spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(path).spawn()?;
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        Command::new("xdg-open").arg(path).spawn()?;
    }

    Ok(())
}
