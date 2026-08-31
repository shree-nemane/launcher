use crate::resolver::{resolve_raw, ResolvedCommand, ResolverError};
use crate::storage::StorageManager;
use tauri::State;

#[tauri::command]
pub fn resolve_command(
    input: String,
    storage: State<'_, StorageManager>,
) -> Result<ResolvedCommand, ResolverError> {
    resolve_raw(&input, &storage)
}
