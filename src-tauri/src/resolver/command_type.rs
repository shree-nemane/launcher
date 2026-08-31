use crate::models::{Application, Project};
use crate::parser::SystemCommandType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ResolvedCommand {
    #[serde(rename_all = "camelCase")]
    Launch {
        project: Option<Project>,
        applications: Vec<Application>,
        is_group: bool,
    },
    #[serde(rename_all = "camelCase")]
    System {
        command: SystemCommandType,
    },
}
