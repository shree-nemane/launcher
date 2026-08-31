use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RunCommand {
    pub name: String,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub command: String,
    pub path: String,
    pub url: Option<String>,
    #[serde(default)]
    pub run_commands: Vec<RunCommand>,
    pub working_directory: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
