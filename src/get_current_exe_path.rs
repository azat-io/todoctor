use std::env;
use std::path::PathBuf;

pub fn get_current_exe_path() -> Option<PathBuf> {
    let current_exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error getting current exe path: {:?}", e);
            return None;
        }
    };
    Some(current_exe_path)
}
