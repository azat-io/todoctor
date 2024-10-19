use std::path::Path;

pub fn identify_not_ignored_file(file: &str, ignores: &[String]) -> bool {
    let file_path = Path::new(file)
        .canonicalize()
        .unwrap_or_else(|_| Path::new(file).to_path_buf());

    let is_ignored = ignores.iter().any(|ignored| {
        let ignored_path = Path::new(ignored)
            .canonicalize()
            .unwrap_or_else(|_| Path::new(ignored).to_path_buf());
        file_path.starts_with(&ignored_path)
    });

    !is_ignored
}
