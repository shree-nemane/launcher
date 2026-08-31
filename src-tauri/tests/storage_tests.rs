use launcher_lib::error::AppError;
use launcher_lib::models::{
    Application, ApplicationGroup, NormalLaunchConfig, Project, ProjectLaunchConfig, RunCommand,
    Settings,
};
use launcher_lib::storage::StorageManager;
use launcher_lib::validation::{validate_application, validate_group, validate_project};
use std::fs::{self, File};
use tempfile::tempdir;

#[test]
fn test_models_serialization_deserialization() {
    let project = Project {
        id: "project_deepfake".to_string(),
        name: "Deepfake Forensic AI".to_string(),
        command: "deepfake".to_string(),
        path: "D:\\Projects\\Deepfake".to_string(),
        url: Some("http://localhost:5173".to_string()),
        run_commands: vec![RunCommand {
            name: "Frontend".to_string(),
            command: "npm run dev".to_string(),
        }],
        working_directory: Some("D:\\Projects\\Deepfake".to_string()),
        created_at: "2026-08-30T00:00:00Z".to_string(),
        updated_at: "2026-08-30T00:00:00Z".to_string(),
    };

    let json = serde_json::to_string_pretty(&project).unwrap();
    assert!(json.contains("\"runCommands\""));
    assert!(json.contains("\"workingDirectory\""));
    assert!(json.contains("\"createdAt\""));

    let deserialized: Project = serde_json::from_str(&json).unwrap();
    assert_eq!(project, deserialized);

    let app = Application {
        id: "app_vscode".to_string(),
        name: "VS Code".to_string(),
        command: "/v".to_string(),
        executable_path: "C:\\Program Files\\Code.exe".to_string(),
        normal_launch: NormalLaunchConfig {
            arguments: vec!["--new-window".to_string()],
        },
        project_launch: Some(ProjectLaunchConfig {
            enabled: true,
            arguments: vec!["{PROJECT_PATH}".to_string()],
        }),
        working_directory: None,
        icon: None,
        created_at: "2026-08-30T00:00:00Z".to_string(),
        updated_at: "2026-08-30T00:00:00Z".to_string(),
    };

    let app_json = serde_json::to_string_pretty(&app).unwrap();
    assert!(app_json.contains("\"executablePath\""));
    assert!(app_json.contains("\"normalLaunch\""));
    assert!(app_json.contains("\"projectLaunch\""));

    let deserialized_app: Application = serde_json::from_str(&app_json).unwrap();
    assert_eq!(app, deserialized_app);
}

#[test]
fn test_default_settings_creation() {
    let settings = Settings::default();
    assert_eq!(settings.global_shortcut, "Alt+Space");
    assert_eq!(settings.default_application_group_id, None);
    assert!(!settings.created_at.is_empty());
    assert!(!settings.updated_at.is_empty());
}

#[test]
fn test_project_validation_rules() {
    let temp_dir = tempdir().unwrap();
    let valid_path = temp_dir.path().to_str().unwrap().to_string();

    // 1. Valid project
    let valid_project = Project {
        id: "proj_1".to_string(),
        name: "Test Project".to_string(),
        command: "myproject".to_string(),
        path: valid_path.clone(),
        url: None,
        run_commands: vec![],
        working_directory: None,
        created_at: "".to_string(),
        updated_at: "".to_string(),
    };
    assert!(validate_project(&valid_project, &[], false).is_ok());

    // 2. Empty name
    let mut empty_name = valid_project.clone();
    empty_name.name = "   ".to_string();
    assert!(matches!(
        validate_project(&empty_name, &[], false),
        Err(AppError::Validation(_))
    ));

    // 3. Empty command
    let mut empty_cmd = valid_project.clone();
    empty_cmd.command = "".to_string();
    assert!(matches!(
        validate_project(&empty_cmd, &[], false),
        Err(AppError::Validation(_))
    ));

    // 4. Command starting with /
    let mut slash_cmd = valid_project.clone();
    slash_cmd.command = "/myproject".to_string();
    assert!(matches!(
        validate_project(&slash_cmd, &[], false),
        Err(AppError::Validation(_))
    ));

    // 5. Reserved command
    let mut reserved_cmd = valid_project.clone();
    reserved_cmd.command = "add-project".to_string();
    assert!(matches!(
        validate_project(&reserved_cmd, &[], false),
        Err(AppError::Validation(_))
    ));

    // 6. Non-existent path
    let mut invalid_path = valid_project.clone();
    invalid_path.path = "Z:\\non_existent_directory_12345".to_string();
    assert!(matches!(
        validate_project(&invalid_path, &[], false),
        Err(AppError::Validation(_))
    ));

    // 7. Path is a file, not a directory
    let temp_file_path = temp_dir.path().join("a_file.txt");
    File::create(&temp_file_path).unwrap();
    let mut file_path_project = valid_project.clone();
    file_path_project.path = temp_file_path.to_str().unwrap().to_string();
    assert!(matches!(
        validate_project(&file_path_project, &[], false),
        Err(AppError::Validation(_))
    ));
}

#[test]
fn test_application_validation_rules() {
    let temp_dir = tempdir().unwrap();
    let exe_file_path = temp_dir.path().join("code.exe");
    File::create(&exe_file_path).unwrap();
    let valid_exe = exe_file_path.to_str().unwrap().to_string();

    // 1. Valid application
    let valid_app = Application {
        id: "app_1".to_string(),
        name: "VS Code".to_string(),
        command: "/v".to_string(),
        executable_path: valid_exe.clone(),
        normal_launch: NormalLaunchConfig::default(),
        project_launch: None,
        working_directory: None,
        icon: None,
        created_at: "".to_string(),
        updated_at: "".to_string(),
    };
    assert!(validate_application(&valid_app, &[], false).is_ok());

    // 2. Command not starting with /
    let mut no_slash = valid_app.clone();
    no_slash.command = "v".to_string();
    assert!(matches!(
        validate_application(&no_slash, &[], false),
        Err(AppError::Validation(_))
    ));

    // 3. Command is //
    let mut double_slash = valid_app.clone();
    double_slash.command = "//".to_string();
    assert!(matches!(
        validate_application(&double_slash, &[], false),
        Err(AppError::Validation(_))
    ));

    // 4. Command contains spaces
    let mut spaced_cmd = valid_app.clone();
    spaced_cmd.command = "/vs code".to_string();
    assert!(matches!(
        validate_application(&spaced_cmd, &[], false),
        Err(AppError::Validation(_))
    ));

    // 5. Non-existent executable
    let mut invalid_exe_app = valid_app.clone();
    invalid_exe_app.executable_path = "Z:\\non_existent_app_12345.exe".to_string();
    assert!(matches!(
        validate_application(&invalid_exe_app, &[], false),
        Err(AppError::Validation(_))
    ));
}

#[test]
fn test_duplicate_command_validation_create_vs_update() {
    let temp_dir = tempdir().unwrap();
    let valid_path = temp_dir.path().to_str().unwrap().to_string();

    let project1 = Project {
        id: "proj_1".to_string(),
        name: "Project 1".to_string(),
        command: "deepfake".to_string(),
        path: valid_path.clone(),
        url: None,
        run_commands: vec![],
        working_directory: None,
        created_at: "".to_string(),
        updated_at: "".to_string(),
    };

    let existing = vec![project1.clone()];

    // Creating a new project with the same command (case-insensitive) should fail
    let project2 = Project {
        id: "proj_2".to_string(),
        name: "Project 2".to_string(),
        command: "DEEPFAKE".to_string(),
        path: valid_path.clone(),
        url: None,
        run_commands: vec![],
        working_directory: None,
        created_at: "".to_string(),
        updated_at: "".to_string(),
    };
    assert!(validate_project(&project2, &existing, false).is_err());

    // Updating project 1 while keeping its same command should succeed
    assert!(validate_project(&project1, &existing, true).is_ok());
}

#[test]
fn test_group_validation() {
    let temp_dir = tempdir().unwrap();
    let exe_path = temp_dir.path().join("test.exe");
    File::create(&exe_path).unwrap();

    let app1 = Application {
        id: "app_1".to_string(),
        name: "App 1".to_string(),
        command: "/a".to_string(),
        executable_path: exe_path.to_str().unwrap().to_string(),
        normal_launch: NormalLaunchConfig::default(),
        project_launch: None,
        working_directory: None,
        icon: None,
        created_at: "".to_string(),
        updated_at: "".to_string(),
    };

    let existing_apps = vec![app1];

    let valid_group = ApplicationGroup {
        id: "group_1".to_string(),
        name: "Dev Group".to_string(),
        applications: vec!["app_1".to_string()],
        execution_order: vec!["app_1".to_string()],
        created_at: "".to_string(),
        updated_at: "".to_string(),
    };
    assert!(validate_group(&valid_group, &existing_apps).is_ok());

    let invalid_group = ApplicationGroup {
        id: "group_2".to_string(),
        name: "Invalid Group".to_string(),
        applications: vec!["app_non_existent".to_string()],
        execution_order: vec![],
        created_at: "".to_string(),
        updated_at: "".to_string(),
    };
    assert!(validate_group(&invalid_group, &existing_apps).is_err());

    // Duplicate application IDs in group
    let dup_group = ApplicationGroup {
        id: "group_3".to_string(),
        name: "Dup Group".to_string(),
        applications: vec!["app_1".to_string(), "app_1".to_string()],
        execution_order: vec!["app_1".to_string()],
        created_at: "".to_string(),
        updated_at: "".to_string(),
    };
    assert!(validate_group(&dup_group, &existing_apps).is_err());
}

#[test]
fn test_referential_integrity_application_and_group_deletion() {
    let temp_dir = tempdir().unwrap();
    let data_dir = temp_dir.path().to_path_buf();
    let storage = StorageManager::new(data_dir).unwrap();

    let exe_path1 = temp_dir.path().join("app1.exe");
    File::create(&exe_path1).unwrap();
    let exe_path2 = temp_dir.path().join("app2.exe");
    File::create(&exe_path2).unwrap();

    let app1 = storage
        .create_application(Application {
            id: "".to_string(),
            name: "App 1".to_string(),
            command: "/a1".to_string(),
            executable_path: exe_path1.to_str().unwrap().to_string(),
            normal_launch: NormalLaunchConfig::default(),
            project_launch: None,
            working_directory: None,
            icon: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    let app2 = storage
        .create_application(Application {
            id: "".to_string(),
            name: "App 2".to_string(),
            command: "/a2".to_string(),
            executable_path: exe_path2.to_str().unwrap().to_string(),
            normal_launch: NormalLaunchConfig::default(),
            project_launch: None,
            working_directory: None,
            icon: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    let group = storage
        .create_group(ApplicationGroup {
            id: "".to_string(),
            name: "Dev Workspace".to_string(),
            applications: vec![app1.id.clone(), app2.id.clone()],
            execution_order: vec![app1.id.clone(), app2.id.clone()],
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    // Set as default group in settings
    let mut settings = storage.get_settings().unwrap();
    settings.default_application_group_id = Some(group.id.clone());
    storage.update_settings(settings).unwrap();

    // 1. Delete app1 -> verify group has app1 removed from applications & execution_order
    storage.delete_application(&app1.id).unwrap();
    let updated_group = storage.get_group(&group.id).unwrap();
    assert_eq!(updated_group.applications, vec![app2.id.clone()]);
    assert_eq!(updated_group.execution_order, vec![app2.id.clone()]);

    // 2. Delete group -> verify settings.default_application_group_id becomes None
    storage.delete_group(&group.id).unwrap();
    let updated_settings = storage.get_settings().unwrap();
    assert_eq!(updated_settings.default_application_group_id, None);
}

#[test]
fn test_storage_manager_crud_and_persistence() {
    let temp_dir = tempdir().unwrap();
    let data_dir = temp_dir.path().to_path_buf();

    // 1. StorageManager initializes files
    let storage = StorageManager::new(data_dir.clone()).unwrap();
    assert!(data_dir.join("projects.json").exists());
    assert!(data_dir.join("applications.json").exists());
    assert!(data_dir.join("groups.json").exists());
    assert!(data_dir.join("settings.json").exists());

    // 2. Project CRUD
    let sub_dir = temp_dir.path().join("my_proj");
    fs::create_dir(&sub_dir).unwrap();

    let created_proj = storage
        .create_project(Project {
            id: "".to_string(),
            name: "Deepfake".to_string(),
            command: "deepfake".to_string(),
            path: sub_dir.to_str().unwrap().to_string(),
            url: Some("http://localhost:3000".to_string()),
            run_commands: vec![],
            working_directory: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    assert!(created_proj.id.starts_with("project_"));
    let fetched_proj = storage.get_project(&created_proj.id).unwrap();
    assert_eq!(fetched_proj.name, "Deepfake");

    // Update project
    let mut to_update = fetched_proj.clone();
    to_update.name = "Deepfake Updated".to_string();
    let updated_proj = storage.update_project(to_update).unwrap();
    assert_eq!(updated_proj.name, "Deepfake Updated");

    // 3. Application CRUD
    let exe_path = temp_dir.path().join("editor.exe");
    File::create(&exe_path).unwrap();

    let created_app = storage
        .create_application(Application {
            id: "".to_string(),
            name: "Editor".to_string(),
            command: "/e".to_string(),
            executable_path: exe_path.to_str().unwrap().to_string(),
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

    assert!(created_app.id.starts_with("app_"));
    let fetched_app = storage.get_application(&created_app.id).unwrap();
    assert_eq!(fetched_app.command, "/e");

    // 4. Group CRUD
    let created_group = storage
        .create_group(ApplicationGroup {
            id: "".to_string(),
            name: "Default Group".to_string(),
            applications: vec![created_app.id.clone()],
            execution_order: vec![created_app.id.clone()],
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    assert!(created_group.id.starts_with("group_"));

    // 5. Settings CRUD
    let initial_settings = storage.get_settings().unwrap();
    assert_eq!(initial_settings.global_shortcut, "Alt+Space");

    let mut new_settings = initial_settings.clone();
    new_settings.default_application_group_id = Some(created_group.id.clone());
    let updated_settings = storage.update_settings(new_settings).unwrap();
    assert_eq!(
        updated_settings.default_application_group_id,
        Some(created_group.id.clone())
    );

    // 6. Persistence across re-initialization
    drop(storage);

    let storage2 = StorageManager::new(data_dir.clone()).unwrap();
    let reloaded_projects = storage2.get_projects().unwrap();
    assert_eq!(reloaded_projects.len(), 1);
    assert_eq!(reloaded_projects[0].name, "Deepfake Updated");

    let reloaded_apps = storage2.get_applications().unwrap();
    assert_eq!(reloaded_apps.len(), 1);
    assert_eq!(reloaded_apps[0].command, "/e");

    let reloaded_settings = storage2.get_settings().unwrap();
    assert_eq!(
        reloaded_settings.default_application_group_id,
        Some(created_group.id.clone())
    );

    // 7. Deletions
    storage2.delete_project(&created_proj.id).unwrap();
    assert_eq!(storage2.get_projects().unwrap().len(), 0);

    storage2.delete_group(&created_group.id).unwrap();
    assert_eq!(storage2.get_groups().unwrap().len(), 0);

    storage2.delete_application(&created_app.id).unwrap();
    assert_eq!(storage2.get_applications().unwrap().len(), 0);
}

#[test]
fn test_application_update_and_id_timestamp_stability() {
    let temp_dir = tempdir().unwrap();
    let data_dir = temp_dir.path().to_path_buf();
    let storage = StorageManager::new(data_dir).unwrap();

    let exe1 = temp_dir.path().join("app1.exe");
    File::create(&exe1).unwrap();
    let exe2 = temp_dir.path().join("app2.exe");
    File::create(&exe2).unwrap();

    let app1 = storage
        .create_application(Application {
            id: "".to_string(),
            name: "App One".to_string(),
            command: "/a1".to_string(),
            executable_path: exe1.to_str().unwrap().to_string(),
            normal_launch: NormalLaunchConfig::default(),
            project_launch: None,
            working_directory: None,
            icon: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    let _app2 = storage
        .create_application(Application {
            id: "".to_string(),
            name: "App Two".to_string(),
            command: "/a2".to_string(),
            executable_path: exe2.to_str().unwrap().to_string(),
            normal_launch: NormalLaunchConfig::default(),
            project_launch: None,
            working_directory: None,
            icon: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    let original_id = app1.id.clone();
    let original_created_at = app1.created_at.clone();

    // 1. Update app1 while keeping the same command -> Must succeed
    let mut updated_app1 = app1.clone();
    updated_app1.name = "App One Renamed".to_string();
    let res = storage.update_application(updated_app1).unwrap();
    assert_eq!(res.id, original_id);
    assert_eq!(res.created_at, original_created_at);
    assert_eq!(res.name, "App One Renamed");

    // 2. Update app1 to have app2's command -> Must fail duplicate validation
    let mut conflicting_app1 = res.clone();
    conflicting_app1.command = "/a2".to_string();
    assert!(storage.update_application(conflicting_app1).is_err());
}

#[test]
fn test_storage_backup_and_recovery_from_corruption() {
    let temp_dir = tempdir().unwrap();
    let data_dir = temp_dir.path().to_path_buf();
    let storage = StorageManager::new(data_dir.clone()).unwrap();

    let sub_dir = temp_dir.path().join("my_proj");
    fs::create_dir(&sub_dir).unwrap();

    let created_proj = storage
        .create_project(Project {
            id: "".to_string(),
            name: "Deepfake Original".to_string(),
            command: "deepfake".to_string(),
            path: sub_dir.to_str().unwrap().to_string(),
            url: None,
            run_commands: vec![],
            working_directory: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .unwrap();

    // 1. Update project so a .bak file is created
    let mut updated_proj = created_proj.clone();
    updated_proj.name = "Deepfake Second State".to_string();
    storage.update_project(updated_proj).unwrap();

    // Verify backup exists
    let backup_path = data_dir.join("projects.json.bak");
    assert!(backup_path.exists());

    // 2. Corrupt the primary projects.json with invalid JSON syntax
    let primary_path = data_dir.join("projects.json");
    fs::write(&primary_path, "{ invalid_json_syntax: [ missing_bracket ").unwrap();

    // 3. get_projects() should automatically recover data from backup
    let recovered_projects = storage.get_projects().unwrap();
    assert_eq!(recovered_projects.len(), 1);
    assert_eq!(recovered_projects[0].command, "deepfake");
}

#[test]
fn test_corrupted_primary_and_backup_preserves_files_and_returns_error() {
    let temp_dir = tempdir().unwrap();
    let data_dir = temp_dir.path().to_path_buf();
    let storage = StorageManager::new(data_dir.clone()).unwrap();

    let primary_path = data_dir.join("projects.json");
    let backup_path = data_dir.join("projects.json.bak");

    // Corrupt both primary and backup
    fs::write(&primary_path, "{ corrupted primary }").unwrap();
    fs::write(&backup_path, "{ corrupted backup }").unwrap();

    // Must return explicit AppError::Storage error
    let res = storage.get_projects();
    assert!(matches!(res, Err(AppError::Storage(_))));

    // Files on disk must NOT be silently overwritten or deleted
    assert!(primary_path.exists());
    assert!(backup_path.exists());
    assert_eq!(fs::read_to_string(&primary_path).unwrap(), "{ corrupted primary }");
    assert_eq!(fs::read_to_string(&backup_path).unwrap(), "{ corrupted backup }");
}
