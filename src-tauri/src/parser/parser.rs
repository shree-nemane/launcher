use crate::parser::command_type::{ParsedCommand, SystemCommandType};
use crate::parser::error::ParseError;
use crate::parser::tokenizer::tokenize;
use std::collections::HashSet;

pub fn parse(input: &str) -> Result<ParsedCommand, ParseError> {
    let tokens = tokenize(input);
    if tokens.is_empty() {
        return Err(ParseError::empty_command());
    }

    // 1. System Command Evaluation
    if let Some(system_cmd) = SystemCommandType::from_token(&tokens[0]) {
        if tokens.len() > 1 {
            return Err(ParseError::system_command_with_arguments(&tokens[0]));
        }
        return Ok(ParsedCommand::System {
            command: system_cmd,
        });
    }

    for token in &tokens {
        if SystemCommandType::from_token(token).is_some() {
            return Err(ParseError::system_command_with_arguments(token));
        }
    }

    // 2. Token Classification and Validation
    let mut project_token: Option<String> = None;
    let mut application_tokens: Vec<String> = Vec::new();
    let mut seen_apps: HashSet<String> = HashSet::new();
    let mut has_default_group = false;

    for (index, token) in tokens.iter().enumerate() {
        if token == "//" {
            if has_default_group {
                return Err(ParseError::mixed_default_group_and_applications());
            }
            has_default_group = true;
        } else if token.starts_with('/') {
            // Check for invalid slash syntax (e.g. '///', '////', '//v', '/' alone, or internal '/')
            if token.starts_with("//") || token.len() == 1 || token[1..].contains('/') {
                return Err(ParseError::invalid_slash_syntax(token));
            }

            if seen_apps.insert(token.clone()) {
                application_tokens.push(token.clone());
            }
        } else {
            // Non-slash token -> Project token
            if let Some(ref existing_project) = project_token {
                return Err(ParseError::multiple_project_commands(
                    existing_project,
                    token,
                ));
            }
            if index > 0 {
                return Err(ParseError::invalid_token_order(token));
            }
            project_token = Some(token.clone());
        }
    }

    // 3. Group vs Application Conflict Validation
    if has_default_group && !application_tokens.is_empty() {
        return Err(ParseError::mixed_default_group_and_applications());
    }

    Ok(ParsedCommand::Launch {
        project_command: project_token,
        application_commands: application_tokens,
        use_default_group: has_default_group,
    })
}
