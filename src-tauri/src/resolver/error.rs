use crate::parser::ParseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolverError {
    pub code: ResolverErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_error: Option<ParseError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResolverErrorCode {
    ParseError,
    ProjectNotFound,
    ApplicationNotFound,
    DefaultGroupNotConfigured,
    GroupNotFound,
    CorruptedGroupData,
    StorageError,
}

impl ResolverError {
    pub fn project_not_found(cmd: &str) -> Self {
        Self {
            code: ResolverErrorCode::ProjectNotFound,
            message: format!("Project '{}' was not found", cmd),
            parse_error: None,
        }
    }

    pub fn application_not_found(cmd: &str) -> Self {
        Self {
            code: ResolverErrorCode::ApplicationNotFound,
            message: format!("Application command '{}' was not found", cmd),
            parse_error: None,
        }
    }

    pub fn default_group_not_configured() -> Self {
        Self {
            code: ResolverErrorCode::DefaultGroupNotConfigured,
            message: "Default application group is not configured in settings".to_string(),
            parse_error: None,
        }
    }

    pub fn group_not_found(id: &str) -> Self {
        Self {
            code: ResolverErrorCode::GroupNotFound,
            message: format!("Configured application group with ID '{}' was not found", id),
            parse_error: None,
        }
    }

    pub fn corrupted_group_data(details: &str) -> Self {
        Self {
            code: ResolverErrorCode::CorruptedGroupData,
            message: format!("Corrupted group data: {}", details),
            parse_error: None,
        }
    }

    pub fn storage_error(msg: &str) -> Self {
        Self {
            code: ResolverErrorCode::StorageError,
            message: format!("Storage error: {}", msg),
            parse_error: None,
        }
    }
}

impl From<ParseError> for ResolverError {
    fn from(err: ParseError) -> Self {
        Self {
            code: ResolverErrorCode::ParseError,
            message: err.message.clone(),
            parse_error: Some(err),
        }
    }
}

impl From<crate::error::AppError> for ResolverError {
    fn from(err: crate::error::AppError) -> Self {
        Self::storage_error(&err.to_string())
    }
}

impl std::fmt::Display for ResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {}", self.code, self.message)
    }
}

impl std::error::Error for ResolverError {}
