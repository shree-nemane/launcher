use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ParsedCommand {
    #[serde(rename_all = "camelCase")]
    Launch {
        project_command: Option<String>,
        application_commands: Vec<String>,
        use_default_group: bool,
    },
    #[serde(rename_all = "camelCase")]
    System {
        command: SystemCommandType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SystemCommandType {
    AddProject,
    AddApp,
    AddGroup,
    ManageProjects,
    ManageApps,
    ManageGroups,
    Help,
    Settings,
}

impl SystemCommandType {
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "add-project" => Some(Self::AddProject),
            "add-app" => Some(Self::AddApp),
            "add-group" => Some(Self::AddGroup),
            "manage-projects" => Some(Self::ManageProjects),
            "manage-apps" => Some(Self::ManageApps),
            "manage-groups" => Some(Self::ManageGroups),
            "help" => Some(Self::Help),
            "settings" | "config" => Some(Self::Settings),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AddProject => "add-project",
            Self::AddApp => "add-app",
            Self::AddGroup => "add-group",
            Self::ManageProjects => "manage-projects",
            Self::ManageApps => "manage-apps",
            Self::ManageGroups => "manage-groups",
            Self::Help => "help",
            Self::Settings => "settings",
        }
    }
}
