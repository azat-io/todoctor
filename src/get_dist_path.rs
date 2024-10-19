use std::env;
use std::fs;
use std::path::PathBuf;

pub fn get_dist_path() -> Option<PathBuf> {
    let exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error getting current exe path: {:?}", e);
            return None;
        }
    };

    if let Ok(real_path) = fs::read_link(&exe_path) {
        let project_root = real_path.parent()?.parent()?.to_path_buf();
        let dist_path = project_root.join("dist");
        if dist_path.exists() {
            return Some(dist_path);
        }
    }

    let project_root = exe_path.parent()?.parent()?.to_path_buf();
    let dist_path = project_root.join("dist");
    if dist_path.exists() {
        return Some(dist_path);
    }

    None
}
