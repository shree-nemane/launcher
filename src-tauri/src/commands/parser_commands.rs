use crate::parser::{parse, ParseError, ParsedCommand};

#[tauri::command]
pub fn parse_command(input: String) -> Result<ParsedCommand, ParseError> {
    parse(&input)
}
