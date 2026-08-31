use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseError {
    pub code: ParseErrorCode,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ParseErrorCode {
    EmptyCommand,
    MultipleProjectCommands,
    InvalidTokenOrder,
    InvalidSlashSyntax,
    MixedDefaultGroupAndApplications,
    SystemCommandWithArguments,
}

impl ParseError {
    pub fn empty_command() -> Self {
        Self {
            code: ParseErrorCode::EmptyCommand,
            message: "Command cannot be empty".to_string(),
        }
    }

    pub fn multiple_project_commands(first: &str, second: &str) -> Self {
        Self {
            code: ParseErrorCode::MultipleProjectCommands,
            message: format!(
                "Multiple project commands are not supported: '{}' and '{}'",
                first, second
            ),
        }
    }

    pub fn invalid_token_order(token: &str) -> Self {
        Self {
            code: ParseErrorCode::InvalidTokenOrder,
            message: format!(
                "Invalid command order: project command '{}' must appear before application commands",
                token
            ),
        }
    }

    pub fn invalid_slash_syntax(token: &str) -> Self {
        Self {
            code: ParseErrorCode::InvalidSlashSyntax,
            message: format!(
                "Invalid slash syntax '{}'. Only single slash (e.g., '/v') and double slash ('//') are permitted",
                token
            ),
        }
    }

    pub fn mixed_default_group_and_applications() -> Self {
        Self {
            code: ParseErrorCode::MixedDefaultGroupAndApplications,
            message: "Cannot mix default group command '//' with individual application commands"
                .to_string(),
        }
    }

    pub fn system_command_with_arguments(system_cmd: &str) -> Self {
        Self {
            code: ParseErrorCode::SystemCommandWithArguments,
            message: format!(
                "System command '{}' does not accept arguments",
                system_cmd
            ),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {}", self.code, self.message)
    }
}

impl std::error::Error for ParseError {}
