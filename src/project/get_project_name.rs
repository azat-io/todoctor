use crate::fs::get_current_directory;

pub fn get_project_name() -> Option<String> {
    if let Some(current_dir) = get_current_directory() {
        if let Some(dir_name) = current_dir.file_name() {
            return dir_name.to_str().map(|s| s.to_string());
        }
    }
    None
}
