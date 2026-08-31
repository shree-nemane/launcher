use crate::execution::{execute_raw, plan_raw, ExecutionPlan, ExecutionResult, PipelineError};
use crate::storage::StorageManager;
use tauri::State;

#[tauri::command]
pub fn plan_command(
    input: String,
    storage: State<'_, StorageManager>,
) -> Result<ExecutionPlan, PipelineError> {
    plan_raw(&input, &storage)
}

#[tauri::command]
pub fn execute_command(
    input: String,
    storage: State<'_, StorageManager>,
) -> Result<ExecutionResult, PipelineError> {
    execute_raw(&input, &storage)
}
