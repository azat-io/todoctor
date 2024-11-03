use std::{env, path::PathBuf};

pub fn get_dist_path() -> Option<PathBuf> {
    env::var("DIST_PATH")
        .ok()
        .map(PathBuf::from)
        .filter(|p| p.exists())
}
