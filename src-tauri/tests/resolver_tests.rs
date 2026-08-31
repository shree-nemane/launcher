use launcher_lib::models::{
    Application, ApplicationGroup, NormalLaunchConfig, Project, ProjectLaunchConfig,
};
use launcher_lib::parser::{ParseErrorCode, SystemCommandType};
use launcher_lib::resolver::{resolve_raw, ResolvedCommand, ResolverErrorCode};
use launcher_lib::storage::StorageManager;
use std::fs::{self, File};
use tempfile::tempdir;

fn setup_test_environment() -> (tempfile::TempDir, StorageManager, Project, Application, Application) {
    let temp_dir = tempdir().unwrap();
    let data_dir = temp_dir.path().to_path_buf();
    let storage = StorageManager::new(data_dir).unwrap();

    let proj_dir = temp_dir.path().join("deepfake_dir");
    fs::create_dir(&proj_dir).unwrap();

    let exe_v = temp_dir.path().join("code.exe");
    File::create(&exe_v).unwrap();

    let exe_b = temp_dir.path().join("browser.exe");
    File::create(&exe_b).unwrap();

    let project = storage
        .create_project(Project {
            id: "".to_string(),
            name: "Deepfake AI".to_string(),
            command: "deepfake".to_string(),
            path: proj_dir.to_str().unwrap().to_string(),
            url: Some("http://localhost:5173".to_string()),
            run_commands: vec![],
            working_directory: None,
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
            normal_launch: NormalLaunchConfig::default(),
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
                arguments: vec!["{PROJECT_URL}".to_string()],
            }),
            working_directory: None,
            icon: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    (temp_dir, storage, project, app_v, app_b)
}

#[test]
fn test_project_and_application_resolution() {
    let (_dir, storage, project, app_v, app_b) = setup_test_environment();

    let resolved = resolve_raw("deepfake /v /b", &storage).unwrap();
    match resolved {
        ResolvedCommand::Launch {
            project: res_proj,
            applications: res_apps,
            is_group,
        } => {
            assert_eq!(res_proj.unwrap().id, project.id);
            assert_eq!(res_apps.len(), 2);
            assert_eq!(res_apps[0].id, app_v.id);
            assert_eq!(res_apps[1].id, app_b.id);
            assert!(!is_group);
        }
        _ => panic!("Expected ResolvedCommand::Launch"),
    }
}

#[test]
fn test_case_insensitive_lookup() {
    let (_dir, storage, project, app_v, _) = setup_test_environment();

    let resolved = resolve_raw("  DEEPFAKE   /V  ", &storage).unwrap();
    match resolved {
        ResolvedCommand::Launch {
            project: res_proj,
            applications: res_apps,
            is_group,
        } => {
            assert_eq!(res_proj.unwrap().id, project.id);
            assert_eq!(res_apps.len(), 1);
            assert_eq!(res_apps[0].id, app_v.id);
            assert!(!is_group);
        }
        _ => panic!("Expected ResolvedCommand::Launch"),
    }
}

#[test]
fn test_project_only_resolution() {
    let (_dir, storage, project, _, _) = setup_test_environment();

    let resolved = resolve_raw("deepfake", &storage).unwrap();
    match resolved {
        ResolvedCommand::Launch {
            project: res_proj,
            applications: res_apps,
            is_group,
        } => {
            assert_eq!(res_proj.unwrap().id, project.id);
            assert!(res_apps.is_empty());
            assert!(!is_group);
        }
        _ => panic!("Expected ResolvedCommand::Launch"),
    }
}

#[test]
fn test_applications_only_resolution() {
    let (_dir, storage, _, app_v, app_b) = setup_test_environment();

    let resolved = resolve_raw("/b /v", &storage).unwrap();
    match resolved {
        ResolvedCommand::Launch {
            project: res_proj,
            applications: res_apps,
            is_group,
        } => {
            assert!(res_proj.is_none());
            assert_eq!(res_apps.len(), 2);
            assert_eq!(res_apps[0].id, app_b.id);
            assert_eq!(res_apps[1].id, app_v.id);
            assert!(!is_group);
        }
        _ => panic!("Expected ResolvedCommand::Launch"),
    }
}

#[test]
fn test_default_group_resolution_and_execution_order() {
    let (_dir, storage, project, app_v, app_b) = setup_test_environment();

    // Create group with execution order: app_b then app_v
    let group = storage
        .create_group(ApplicationGroup {
            id: "".to_string(),
            name: "Dev Workspace".to_string(),
            applications: vec![app_v.id.clone(), app_b.id.clone()],
            execution_order: vec![app_b.id.clone(), app_v.id.clone()],
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    let mut settings = storage.get_settings().unwrap();
    settings.default_application_group_id = Some(group.id.clone());
    storage.update_settings(settings).unwrap();

    // 1. Resolve "//" (default group only)
    let resolved = resolve_raw("//", &storage).unwrap();
    match resolved {
        ResolvedCommand::Launch {
            project: res_proj,
            applications: res_apps,
            is_group,
        } => {
            assert!(res_proj.is_none());
            assert!(is_group);
            assert_eq!(res_apps.len(), 2);
            assert_eq!(res_apps[0].id, app_b.id); // Execution order first
            assert_eq!(res_apps[1].id, app_v.id); // Execution order second
        }
        _ => panic!("Expected ResolvedCommand::Launch"),
    }

    // 2. Resolve "deepfake //" (project + default group)
    let resolved_proj_group = resolve_raw("deepfake //", &storage).unwrap();
    match resolved_proj_group {
        ResolvedCommand::Launch {
            project: res_proj,
            applications: res_apps,
            is_group,
        } => {
            assert_eq!(res_proj.unwrap().id, project.id);
            assert!(is_group);
            assert_eq!(res_apps.len(), 2);
            assert_eq!(res_apps[0].id, app_b.id);
            assert_eq!(res_apps[1].id, app_v.id);
        }
        _ => panic!("Expected ResolvedCommand::Launch"),
    }
}

#[test]
fn test_system_command_bypasses_storage() {
    let (_dir, storage, _, _, _) = setup_test_environment();

    let resolved = resolve_raw("add-project", &storage).unwrap();
    assert_eq!(
        resolved,
        ResolvedCommand::System {
            command: SystemCommandType::AddProject,
        }
    );

    let resolved_app = resolve_raw("add-app", &storage).unwrap();
    assert_eq!(
        resolved_app,
        ResolvedCommand::System {
            command: SystemCommandType::AddApp,
        }
    );

    let resolved_mp = resolve_raw("manage-projects", &storage).unwrap();
    assert_eq!(
        resolved_mp,
        ResolvedCommand::System {
            command: SystemCommandType::ManageProjects,
        }
    );

    let resolved_ma = resolve_raw("manage-apps", &storage).unwrap();
    assert_eq!(
        resolved_ma,
        ResolvedCommand::System {
            command: SystemCommandType::ManageApps,
        }
    );
}

#[test]
fn test_unknown_project_and_unknown_application() {
    let (_dir, storage, _, _, _) = setup_test_environment();

    let err = resolve_raw("nonexistent /v", &storage).unwrap_err();
    assert_eq!(err.code, ResolverErrorCode::ProjectNotFound);
    assert!(err.message.contains("nonexistent"));

    let err_app = resolve_raw("deepfake /x", &storage).unwrap_err();
    assert_eq!(err_app.code, ResolverErrorCode::ApplicationNotFound);
    assert!(err_app.message.contains("/x"));
}

#[test]
fn test_missing_default_group_and_missing_group_record() {
    let (_dir, storage, _, _, _) = setup_test_environment();

    // 1. Settings has no default group configured
    let err = resolve_raw("//", &storage).unwrap_err();
    assert_eq!(err.code, ResolverErrorCode::DefaultGroupNotConfigured);

    // 2. Settings points to non-existent group ID
    let mut settings = storage.get_settings().unwrap();
    settings.default_application_group_id = Some("group_non_existent_12345".to_string());
    storage.update_settings(settings).unwrap();

    let err_group = resolve_raw("//", &storage).unwrap_err();
    assert_eq!(err_group.code, ResolverErrorCode::GroupNotFound);
}

#[test]
fn test_parser_error_details_preserved() {
    let (_dir, storage, _, _, _) = setup_test_environment();

    let err = resolve_raw("///", &storage).unwrap_err();
    assert_eq!(err.code, ResolverErrorCode::ParseError);
    assert!(err.parse_error.is_some());
    assert_eq!(
        err.parse_error.unwrap().code,
        ParseErrorCode::InvalidSlashSyntax
    );

    let err_empty = resolve_raw("   ", &storage).unwrap_err();
    assert_eq!(err_empty.code, ResolverErrorCode::ParseError);
    assert_eq!(
        err_empty.parse_error.unwrap().code,
        ParseErrorCode::EmptyCommand
    );
}
