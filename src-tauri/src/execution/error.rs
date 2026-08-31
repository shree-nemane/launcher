use crate::parser::ParseError;
use crate::resolver::ResolverError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineError {
    pub code: PipelineErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_error: Option<ParseError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_error: Option<ResolverError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PipelineErrorCode {
    ParseError,
    ResolverError,
    PlanningError,
    StorageError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanningError {
    MissingProjectUrl {
        project_name: String,
        template: String,
    },
    InvalidWorkingDirectory {
        path: String,
    },
}

impl std::fmt::Display for PlanningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanningError::MissingProjectUrl {
                project_name,
                template,
            } => write!(
                f,
                "Project '{}' has no URL configured, but argument '{}' requires {{PROJECT_URL}}",
                project_name, template
            ),
            PlanningError::InvalidWorkingDirectory { path } => {
                write!(f, "Invalid working directory: '{}'", path)
            }
        }
    }
}

impl std::error::Error for PlanningError {}

impl From<ParseError> for PipelineError {
    fn from(err: ParseError) -> Self {
        Self {
            code: PipelineErrorCode::ParseError,
            message: err.message.clone(),
            parse_error: Some(err),
            resolver_error: None,
        }
    }
}

impl From<ResolverError> for PipelineError {
    fn from(err: ResolverError) -> Self {
        Self {
            code: PipelineErrorCode::ResolverError,
            message: err.message.clone(),
            parse_error: err.parse_error.clone(),
            resolver_error: Some(err),
        }
    }
}

impl From<PlanningError> for PipelineError {
    fn from(err: PlanningError) -> Self {
        Self {
            code: PipelineErrorCode::PlanningError,
            message: err.to_string(),
            parse_error: None,
            resolver_error: None,
        }
    }
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {}", self.code, self.message)
    }
}

impl std::error::Error for PipelineError {}
