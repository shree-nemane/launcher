use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NormalLaunchConfig {
    #[serde(default)]
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectLaunchConfig {
    pub enabled: bool,
    #[serde(default)]
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub id: String,
    pub name: String,
    pub command: String,
    pub executable_path: String,
    pub normal_launch: NormalLaunchConfig,
    pub project_launch: Option<ProjectLaunchConfig>,
    pub working_directory: Option<String>,
    pub icon: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
