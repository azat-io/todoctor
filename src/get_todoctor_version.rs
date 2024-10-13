use crate::get_current_exe_path::get_current_exe_path;
use serde_json::Value;
use tokio::fs;

pub async fn get_todoctor_version() -> Option<String> {
    let current_exe_path = match get_current_exe_path() {
        Some(path) => path,
        None => {
            eprintln!("Error: could not get current exe path.");
            return None;
        }
    };

    let parent_dir = match current_exe_path.parent().and_then(|p| p.parent()) {
        Some(path) => path,
        None => {
            eprintln!("Error: could not get parent directory.");
            return None;
        }
    };

    let package_json_path = parent_dir.join("package.json");

    let package_json_content = match fs::read_to_string(&package_json_path).await {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading package.json: {:?}", e);
            return None;
        }
    };

    let package_json: Value = match serde_json::from_str(&package_json_content) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error parsing package.json: {:?}", e);
            return None;
        }
    };

    package_json["version"].as_str().map(|v| v.to_string())
}
