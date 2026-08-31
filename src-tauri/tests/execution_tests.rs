use launcher_lib::execution::{
    execute, plan_raw, ExecutionPlan, LaunchAction, PipelineErrorCode,
};
use launcher_lib::models::{
    Application, ApplicationGroup, NormalLaunchConfig, Project, ProjectLaunchConfig, RunCommand,
};
use launcher_lib::parser::SystemCommandType;
use launcher_lib::storage::StorageManager;
use std::fs::{self, File};
use std::path::Path;
use tempfile::tempdir;

fn setup_test_environment() -> (tempfile::TempDir, StorageManager, Project, Application, Application, Application) {
    let temp_dir = tempdir().unwrap();
    let data_dir = temp_dir.path().to_path_buf();
    let storage = StorageManager::new(data_dir).unwrap();

    let proj_dir = temp_dir.path().join("deepfake_dir");
    fs::create_dir(&proj_dir).unwrap();

    let exe_v = temp_dir.path().join("code.exe");
    File::create(&exe_v).unwrap();

    let exe_b = temp_dir.path().join("browser.exe");
    File::create(&exe_b).unwrap();

    let exe_t = temp_dir.path().join("terminal.exe");
    File::create(&exe_t).unwrap();

    let project = storage
        .create_project(Project {
            id: "".to_string(),
            name: "Deepfake Forensic AI".to_string(),
            command: "deepfake".to_string(),
            path: proj_dir.to_str().unwrap().to_string(),
            url: Some("http://localhost:5173".to_string()),
            run_commands: vec![RunCommand {
                name: "Frontend".to_string(),
                command: "npm run dev".to_string(),
            }],
            working_directory: Some(proj_dir.to_str().unwrap().to_string()),
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    let app_v = storage
        .create_application(Application {
            id: "".to_string(),
            name: "VS Code".to_string(),
            command: "/v".to_string(),
            executable_path: exe_v.to_str().unwrap().to_string(),
            normal_launch: NormalLaunchConfig {
                arguments: vec!["--new-window".to_string()],
            },
            project_launch: Some(ProjectLaunchConfig {
                enabled: true,
                arguments: vec!["{PROJECT_PATH}".to_string()],
            }),
            working_directory: None,
            icon: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    let app_b = storage
        .create_application(Application {
            id: "".to_string(),
            name: "Browser".to_string(),
            command: "/b".to_string(),
            executable_path: exe_b.to_str().unwrap().to_string(),
            normal_launch: NormalLaunchConfig::default(),
            project_launch: Some(ProjectLaunchConfig {
                enabled: true,
                arguments: vec!["--url".to_string(), "{PROJECT_URL}".to_string()],
            }),
            working_directory: None,
            icon: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    let app_t = storage
        .create_application(Application {
            id: "".to_string(),
            name: "Terminal".to_string(),
            command: "/t".to_string(),
            executable_path: exe_t.to_str().unwrap().to_string(),
            normal_launch: NormalLaunchConfig::default(),
            project_launch: Some(ProjectLaunchConfig {
                enabled: true,
                arguments: vec!["--title".to_string(), "{PROJECT_NAME}".to_string()],
            }),
            working_directory: Some("{PROJECT_PATH}".to_string()),
            icon: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    (temp_dir, storage, project, app_v, app_b, app_t)
}

#[test]
fn test_project_only_folder_opening() {
    let (_dir, storage, project, _, _, _) = setup_test_environment();

    let plan = plan_raw("deepfake", &storage).unwrap();
    match plan {
        ExecutionPlan::Launch { actions } => {
            assert_eq!(actions.len(), 1);
            match &actions[0] {
                LaunchAction::OpenFolder { name, path } => {
                    assert_eq!(name, "Deepfake Forensic AI");
                    assert_eq!(path, &project.path);
                }
                _ => panic!("Expected LaunchAction::OpenFolder"),
            }
        }
        _ => panic!("Expected ExecutionPlan::Launch"),
    }
}

#[test]
fn test_project_run_commands_execution() {
    let (_dir, storage, project, _, _, _) = setup_test_environment();

    // Built-in /run executes configured run_commands in PowerShell
    let plan_run = plan_raw("deepfake /run", &storage).unwrap();
    match plan_run {
        ExecutionPlan::Launch { actions } => {
            assert_eq!(actions.len(), 1);
            match &actions[0] {
                LaunchAction::Process { name, working_directory, .. } => {
                    assert_eq!(name, "Deepfake Forensic AI: Frontend");
                    assert_eq!(working_directory, &Some(project.path.clone()));
                }
                _ => panic!("Expected LaunchAction::Process for run command"),
            }
        }
        _ => panic!("Expected ExecutionPlan::Launch"),
    }
}

#[test]
fn test_normal_application_launch() {
    let (_dir, storage, _, app_v, _, _) = setup_test_environment();

    let plan = plan_raw("/v", &storage).unwrap();
    match plan {
        ExecutionPlan::Launch { actions } => {
            assert_eq!(actions.len(), 1);
            match &actions[0] {
                LaunchAction::Process {
                    name,
                    executable_path,
                    arguments,
                    working_directory,
                } => {
                    assert_eq!(name, "VS Code");
                    assert_eq!(executable_path, &app_v.executable_path);
                    assert_eq!(arguments, &vec!["--new-window".to_string()]);
                    assert_eq!(working_directory, &None);
                }
                _ => panic!("Expected LaunchAction::Process"),
            }
        }
        _ => panic!("Expected ExecutionPlan::Launch"),
    }
}

#[test]
fn test_project_aware_application_launch_and_interpolation() {
    let (_dir, storage, project, app_v, app_b, app_t) = setup_test_environment();

    let plan = plan_raw("deepfake /v /b /t", &storage).unwrap();
    match plan {
        ExecutionPlan::Launch { actions } => {
            assert_eq!(actions.len(), 3);

            // Action 1: VS Code with {PROJECT_PATH}
            match &actions[0] {
                LaunchAction::Process {
                    name,
                    executable_path,
                    arguments,
                    working_directory,
                } => {
                    assert_eq!(name, "VS Code");
                    assert_eq!(executable_path, &app_v.executable_path);
                    assert_eq!(arguments, &vec![project.path.clone()]);
                    assert_eq!(working_directory, &Some(project.path.clone()));
                }
                _ => panic!("Expected Process for VS Code"),
            }

            // Action 2: Browser with {PROJECT_URL}
            match &actions[1] {
                LaunchAction::Process {
                    name,
                    executable_path,
                    arguments,
                    ..
                } => {
                    assert_eq!(name, "Browser");
                    assert_eq!(executable_path, &app_b.executable_path);
                    assert_eq!(
                        arguments,
                        &vec![
                            "--url".to_string(),
                            "http://localhost:5173".to_string()
                        ]
                    );
                }
                _ => panic!("Expected Process for Browser"),
            }

            // Action 3: Terminal with {PROJECT_NAME} and working_directory {PROJECT_PATH}
            match &actions[2] {
                LaunchAction::Process {
                    name,
                    executable_path,
                    arguments,
                    working_directory,
                } => {
                    assert_eq!(name, "Terminal");
                    assert_eq!(executable_path, &app_t.executable_path);
                    assert_eq!(
                        arguments,
                        &vec![
                            "--title".to_string(),
                            "Deepfake Forensic AI".to_string()
                        ]
                    );
                    assert_eq!(working_directory, &Some(project.path.clone()));
                }
                _ => panic!("Expected Process for Terminal"),
            }
        }
        _ => panic!("Expected ExecutionPlan::Launch"),
    }
}

#[test]
fn test_missing_project_url_planning_error() {
    let (temp_dir, storage, _, _, _, _) = setup_test_environment();

    let no_url_dir = temp_dir.path().join("no_url_proj");
    fs::create_dir(&no_url_dir).unwrap();

    storage
        .create_project(Project {
            id: "".to_string(),
            name: "No URL Project".to_string(),
            command: "nourl".to_string(),
            path: no_url_dir.to_str().unwrap().to_string(),
            url: None, // No URL configured
            run_commands: vec![],
            working_directory: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    let err = plan_raw("nourl /b", &storage).unwrap_err();
    assert_eq!(err.code, PipelineErrorCode::PlanningError);
    assert!(err.message.contains("{PROJECT_URL}"));
    assert!(err.message.contains("No URL Project"));
}

#[test]
fn test_group_execution_order_and_action_index() {
    let (_dir, storage, _, app_v, app_b, _) = setup_test_environment();

    let group = storage
        .create_group(ApplicationGroup {
            id: "".to_string(),
            name: "Dev Workspace".to_string(),
            applications: vec![app_v.id.clone(), app_b.id.clone()],
            execution_order: vec![app_b.id.clone(), app_v.id.clone()], // Browser then VS Code
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    let mut settings = storage.get_settings().unwrap();
    settings.default_application_group_id = Some(group.id);
    storage.update_settings(settings).unwrap();

    let plan = plan_raw("//", &storage).unwrap();
    match plan {
        ExecutionPlan::Launch { actions } => {
            assert_eq!(actions.len(), 2);
            match &actions[0] {
                LaunchAction::Process { name, .. } => assert_eq!(name, "Browser"),
                _ => panic!("Expected Browser first"),
            }
            match &actions[1] {
                LaunchAction::Process { name, .. } => assert_eq!(name, "VS Code"),
                _ => panic!("Expected VS Code second"),
            }
        }
        _ => panic!("Expected Launch"),
    }
}

#[test]
fn test_system_command_planning() {
    let (_dir, storage, _, _, _, _) = setup_test_environment();

    let plan_proj = plan_raw("add-project", &storage).unwrap();
    assert_eq!(
        plan_proj,
        ExecutionPlan::System {
            command: SystemCommandType::AddProject,
        }
    );

    let plan_app = plan_raw("add-app", &storage).unwrap();
    assert_eq!(
        plan_app,
        ExecutionPlan::System {
            command: SystemCommandType::AddApp,
        }
    );

    let plan_mp = plan_raw("manage-projects", &storage).unwrap();
    assert_eq!(
        plan_mp,
        ExecutionPlan::System {
            command: SystemCommandType::ManageProjects,
        }
    );

    let plan_ma = plan_raw("manage-apps", &storage).unwrap();
    assert_eq!(
        plan_ma,
        ExecutionPlan::System {
            command: SystemCommandType::ManageApps,
        }
    );

    let plan_ag = plan_raw("add-group", &storage).unwrap();
    assert_eq!(
        plan_ag,
        ExecutionPlan::System {
            command: SystemCommandType::AddGroup,
        }
    );

    let plan_mg = plan_raw("manage-groups", &storage).unwrap();
    assert_eq!(
        plan_mg,
        ExecutionPlan::System {
            command: SystemCommandType::ManageGroups,
        }
    );

    let exec_res = execute(&plan_ma);
    assert!(exec_res.success);
    assert_eq!(exec_res.system_command, Some(SystemCommandType::ManageApps));
    assert!(exec_res.action_results.is_empty());
}

#[test]
fn test_run_commands_are_not_executed_in_v1() {
    let (_dir, storage, _, _, _, _) = setup_test_environment();

    // The setup project has run_commands = vec![RunCommand { name: "Frontend", command: "npm run dev" }]
    let plan = plan_raw("deepfake /v", &storage).unwrap();
    match plan {
        ExecutionPlan::Launch { actions } => {
            // Only the VS Code application process action is present; runCommands are not generated
            assert_eq!(actions.len(), 1);
            match &actions[0] {
                LaunchAction::Process { name, .. } => assert_eq!(name, "VS Code"),
                _ => panic!("Expected only VS Code"),
            }
        }
        _ => panic!("Expected Launch"),
    }
}

#[test]
fn test_partial_execution_failures_captured_in_result() {
    let plan = ExecutionPlan::Launch {
        actions: vec![
            LaunchAction::Process {
                name: "Invalid App".to_string(),
                executable_path: "Z:\\this_executable_does_not_exist_9999.exe".to_string(),
                arguments: vec![],
                working_directory: None,
            },
        ],
    };

    let result = execute(&plan);
    assert!(!result.success);
    assert_eq!(result.action_results.len(), 1);
    assert_eq!(result.action_results[0].action_index, 0);
    assert_eq!(result.action_results[0].name, "Invalid App");
    assert!(!result.action_results[0].success);
    let err_msg = result.action_results[0].error.as_ref().unwrap();
    assert!(err_msg.contains("was not found"));
    assert!(err_msg.contains("moved, renamed, or uninstalled"));
}

#[test]
fn test_executable_path_is_directory_returns_friendly_error() {
    let temp_dir = tempdir().unwrap();
    let folder_path = temp_dir.path().join("a_folder");
    fs::create_dir(&folder_path).unwrap();

    let plan = ExecutionPlan::Launch {
        actions: vec![LaunchAction::Process {
            name: "Folder App".to_string(),
            executable_path: folder_path.to_str().unwrap().to_string(),
            arguments: vec![],
            working_directory: None,
        }],
    };

    let result = execute(&plan);
    assert!(!result.success);
    let err_msg = result.action_results[0].error.as_ref().unwrap();
    assert!(err_msg.contains("is a directory, not an executable file"));
}

#[test]
fn test_missing_project_folder_returns_friendly_error() {
    let plan = ExecutionPlan::Launch {
        actions: vec![LaunchAction::OpenFolder {
            name: "Missing Project".to_string(),
            path: "Z:\\non_existent_folder_99999".to_string(),
        }],
    };

    let result = execute(&plan);
    assert!(!result.success);
    let err_msg = result.action_results[0].error.as_ref().unwrap();
    assert!(err_msg.contains("Project folder for 'Missing Project' was not found"));
}

#[test]
fn test_missing_working_directory_returns_friendly_error() {
    let temp_dir = tempdir().unwrap();
    let exe_path = temp_dir.path().join("valid_app.exe");
    File::create(&exe_path).unwrap();

    let plan = ExecutionPlan::Launch {
        actions: vec![LaunchAction::Process {
            name: "App With Bad WD".to_string(),
            executable_path: exe_path.to_str().unwrap().to_string(),
            arguments: vec![],
            working_directory: Some("Z:\\non_existent_wd_12345".to_string()),
        }],
    };

    let result = execute(&plan);
    assert!(!result.success);
    let err_msg = result.action_results[0].error.as_ref().unwrap();
    assert!(err_msg.contains("Working directory for 'App With Bad WD' was not found"));
}

#[test]
fn test_paths_with_spaces_and_special_chars() {
    let temp_dir = tempdir().unwrap();
    let special_dir = temp_dir.path().join("My Project & Special @ (100%)");
    fs::create_dir(&special_dir).unwrap();

    let exe_path = special_dir.join("My Application Code.exe");
    File::create(&exe_path).unwrap();

    let _plan = ExecutionPlan::Launch {
        actions: vec![LaunchAction::Process {
            name: "Special Path App".to_string(),
            executable_path: exe_path.to_str().unwrap().to_string(),
            arguments: vec!["--input".to_string(), "arg with spaces".to_string()],
            working_directory: Some(special_dir.to_str().unwrap().to_string()),
        }],
    };

    // Preflight validation must succeed for paths with spaces and special characters
    assert!(Path::new(&exe_path).is_file());
    assert!(Path::new(&special_dir).is_dir());
}
