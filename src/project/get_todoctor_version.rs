use crate::fs::get_dist_path;
use serde_json::Value;
use tokio::fs;

pub async fn get_todoctor_version() -> Option<String> {
    let dist_path = match get_dist_path() {
        Some(path) => path,
        None => {
            eprintln!("Error: could not get dist directory path.");
            return None;
        }
    };

    let package_json_path = match dist_path.parent() {
        Some(parent_dir) => parent_dir.join("package.json"),
        None => {
            eprintln!("Error: could not get parent directory of dist.");
            return None;
        }
    };

    if !package_json_path.exists() {
        eprintln!(
            "Error: package.json not found at {}",
            package_json_path.display()
        );
        return None;
    }

    let package_json_content =
        match fs::read_to_string(&package_json_path).await {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading package.json: {:?}", e);
                return None;
            }
        };

    let package_json: Value = match serde_json::from_str(&package_json_content)
    {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error parsing package.json: {:?}", e);
            return None;
        }
    };

    package_json["version"].as_str().map(|v| v.to_string())
}
