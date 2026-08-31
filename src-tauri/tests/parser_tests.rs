use launcher_lib::parser::{parse, ParseErrorCode, ParsedCommand, SystemCommandType};

#[test]
fn test_project_only_command() {
    let result = parse("deepfake").unwrap();
    assert_eq!(
        result,
        ParsedCommand::Launch {
            project_command: Some("deepfake".to_string()),
            application_commands: vec![],
            use_default_group: false,
        }
    );
}

#[test]
fn test_single_application_command() {
    let result = parse("/v").unwrap();
    assert_eq!(
        result,
        ParsedCommand::Launch {
            project_command: None,
            application_commands: vec!["/v".to_string()],
            use_default_group: false,
        }
    );
}

#[test]
fn test_multiple_application_commands() {
    let result = parse("/a /v /b").unwrap();
    assert_eq!(
        result,
        ParsedCommand::Launch {
            project_command: None,
            application_commands: vec!["/a".to_string(), "/v".to_string(), "/b".to_string()],
            use_default_group: false,
        }
    );
}

#[test]
fn test_project_with_single_application() {
    let result = parse("deepfake /v").unwrap();
    assert_eq!(
        result,
        ParsedCommand::Launch {
            project_command: Some("deepfake".to_string()),
            application_commands: vec!["/v".to_string()],
            use_default_group: false,
        }
    );
}

#[test]
fn test_project_with_multiple_applications() {
    let result = parse("deepfake /a /v /b").unwrap();
    assert_eq!(
        result,
        ParsedCommand::Launch {
            project_command: Some("deepfake".to_string()),
            application_commands: vec!["/a".to_string(), "/v".to_string(), "/b".to_string()],
            use_default_group: false,
        }
    );
}

#[test]
fn test_default_group_command() {
    let result = parse("//").unwrap();
    assert_eq!(
        result,
        ParsedCommand::Launch {
            project_command: None,
            application_commands: vec![],
            use_default_group: true,
        }
    );
}

#[test]
fn test_project_with_default_group() {
    let result = parse("deepfake //").unwrap();
    assert_eq!(
        result,
        ParsedCommand::Launch {
            project_command: Some("deepfake".to_string()),
            application_commands: vec![],
            use_default_group: true,
        }
    );
}

#[test]
fn test_system_commands() {
    let add_project = parse("add-project").unwrap();
    assert_eq!(
        add_project,
        ParsedCommand::System {
            command: SystemCommandType::AddProject,
        }
    );

    let add_app = parse("add-app").unwrap();
    assert_eq!(
        add_app,
        ParsedCommand::System {
            command: SystemCommandType::AddApp,
        }
    );

    let manage_projects = parse("manage-projects").unwrap();
    assert_eq!(
        manage_projects,
        ParsedCommand::System {
            command: SystemCommandType::ManageProjects,
        }
    );

    let manage_apps = parse("manage-apps").unwrap();
    assert_eq!(
        manage_apps,
        ParsedCommand::System {
            command: SystemCommandType::ManageApps,
        }
    );

    let add_group = parse("add-group").unwrap();
    assert_eq!(
        add_group,
        ParsedCommand::System {
            command: SystemCommandType::AddGroup,
        }
    );

    let manage_groups = parse("manage-groups").unwrap();
    assert_eq!(
        manage_groups,
        ParsedCommand::System {
            command: SystemCommandType::ManageGroups,
        }
    );
}

#[test]
fn test_whitespace_and_case_normalization() {
    let result = parse("   DEEPFAKE    /V    /B   ").unwrap();
    assert_eq!(
        result,
        ParsedCommand::Launch {
            project_command: Some("deepfake".to_string()),
            application_commands: vec!["/v".to_string(), "/b".to_string()],
            use_default_group: false,
        }
    );

    let system_result = parse("   ADD-PROJECT   ").unwrap();
    assert_eq!(
        system_result,
        ParsedCommand::System {
            command: SystemCommandType::AddProject,
        }
    );
}

#[test]
fn test_application_deduplication_preserving_order() {
    let result = parse("/v /b /v /a /b").unwrap();
    assert_eq!(
        result,
        ParsedCommand::Launch {
            project_command: None,
            application_commands: vec!["/v".to_string(), "/b".to_string(), "/a".to_string()],
            use_default_group: false,
        }
    );

    let proj_result = parse("deepfake /v /v /v").unwrap();
    assert_eq!(
        proj_result,
        ParsedCommand::Launch {
            project_command: Some("deepfake".to_string()),
            application_commands: vec!["/v".to_string()],
            use_default_group: false,
        }
    );
}

#[test]
fn test_syntax_errors() {
    // 1. Empty command
    let err = parse("   ").unwrap_err();
    assert_eq!(err.code, ParseErrorCode::EmptyCommand);

    // 2. Multiple projects
    let err = parse("deepfake launcher /v").unwrap_err();
    assert_eq!(err.code, ParseErrorCode::MultipleProjectCommands);

    // 3. Invalid token order (project after application)
    let err = parse("/v deepfake").unwrap_err();
    assert_eq!(err.code, ParseErrorCode::InvalidTokenOrder);

    // 4. Invalid slash syntax
    let err1 = parse("///").unwrap_err();
    assert_eq!(err1.code, ParseErrorCode::InvalidSlashSyntax);

    let err2 = parse("////").unwrap_err();
    assert_eq!(err2.code, ParseErrorCode::InvalidSlashSyntax);

    let err3 = parse("//v").unwrap_err();
    assert_eq!(err3.code, ParseErrorCode::InvalidSlashSyntax);

    let err4 = parse("/").unwrap_err();
    assert_eq!(err4.code, ParseErrorCode::InvalidSlashSyntax);

    let err5 = parse("/v/b").unwrap_err();
    assert_eq!(err5.code, ParseErrorCode::InvalidSlashSyntax);

    // 5. Mixed default group and individual applications
    let err = parse("// /v").unwrap_err();
    assert_eq!(err.code, ParseErrorCode::MixedDefaultGroupAndApplications);

    let err = parse("deepfake // /v").unwrap_err();
    assert_eq!(err.code, ParseErrorCode::MixedDefaultGroupAndApplications);

    let err = parse("/v //").unwrap_err();
    assert_eq!(err.code, ParseErrorCode::MixedDefaultGroupAndApplications);

    let err = parse("// //").unwrap_err();
    assert_eq!(err.code, ParseErrorCode::MixedDefaultGroupAndApplications);

    // 6. System command with arguments
    let err = parse("add-project /v").unwrap_err();
    assert_eq!(err.code, ParseErrorCode::SystemCommandWithArguments);

    let err = parse("deepfake add-project").unwrap_err();
    assert_eq!(err.code, ParseErrorCode::SystemCommandWithArguments);
}
