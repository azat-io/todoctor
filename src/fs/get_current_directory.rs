use std::{env, path::PathBuf};

pub fn get_current_directory() -> Option<PathBuf> {
    match env::current_dir() {
        Ok(path) => Some(path),
        Err(e) => {
            eprintln!("Error: Cannot get current directory: {:?}", e);
            None
        }
    }
}
