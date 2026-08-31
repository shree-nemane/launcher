use crate::parser::SystemCommandType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ExecutionPlan {
    #[serde(rename_all = "camelCase")]
    Launch {
        actions: Vec<LaunchAction>,
    },
    #[serde(rename_all = "camelCase")]
    System {
        command: SystemCommandType,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum LaunchAction {
    #[serde(rename_all = "camelCase")]
    Process {
        name: String,
        executable_path: String,
        arguments: Vec<String>,
        working_directory: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    OpenFolder {
        name: String,
        path: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionResult {
    pub success: bool,
    pub action_results: Vec<ExecutionActionResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_command: Option<SystemCommandType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionActionResult {
    pub action_index: usize,
    pub name: String,
    pub success: bool,
    pub error: Option<String>,
}
