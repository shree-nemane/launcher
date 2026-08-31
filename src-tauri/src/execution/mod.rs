pub mod command_type;
pub mod error;
pub mod executor;
pub mod interpolation;
pub mod planner;

pub use command_type::{ExecutionActionResult, ExecutionPlan, ExecutionResult, LaunchAction};
pub use error::{PipelineError, PipelineErrorCode, PlanningError};
pub use executor::execute;
pub use interpolation::{interpolate_arguments, interpolate_string};
pub use planner::plan;

use crate::resolver::resolve_raw;
use crate::storage::StorageManager;

pub fn plan_raw(
    input: &str,
    storage: &StorageManager,
) -> Result<ExecutionPlan, PipelineError> {
    let resolved = resolve_raw(input, storage)?;
    let execution_plan = plan(&resolved)?;
    Ok(execution_plan)
}

pub fn execute_raw(
    input: &str,
    storage: &StorageManager,
) -> Result<ExecutionResult, PipelineError> {
    let execution_plan = plan_raw(input, storage)?;
    let result = execute(&execution_plan);
    Ok(result)
}
