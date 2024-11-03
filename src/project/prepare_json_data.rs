use crate::fs::get_current_directory;
use crate::project::get_project_name;
use crate::types::{TodoHistory, TodoWithBlame};
use serde_json::json;

pub async fn prepare_json_data(
    todo_history_data: &[TodoHistory],
    todos_with_blame: &[TodoWithBlame],
    version: &str,
) -> serde_json::Value {
    let current_directory = get_current_directory()
        .expect("Error: Could not get current directory.");
    let project_name =
        get_project_name().unwrap_or_else(|| "Unknown Project".to_string());

    json!({
        "currentPath": current_directory,
        "history": todo_history_data,
        "data": todos_with_blame,
        "name": project_name,
        "version": version,
    })
}
