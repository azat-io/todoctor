use std::{env, fs, path::PathBuf};

pub fn get_dist_path() -> Option<PathBuf> {
    let exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error getting current exe path: {:?}", e);
            return None;
        }
    };
    let real_exe_path = fs::canonicalize(&exe_path).unwrap_or(exe_path);

    fn ascend_path(mut path: PathBuf, levels: usize) -> Option<PathBuf> {
        for _ in 0..levels {
            path = path.parent()?.to_path_buf();
        }
        Some(path)
    }

    let levels_up = 4;

    let project_root = ascend_path(real_exe_path, levels_up)?;
    let dist_path = project_root.join("dist");
    if dist_path.exists() {
        return Some(dist_path);
    }

    None
}
