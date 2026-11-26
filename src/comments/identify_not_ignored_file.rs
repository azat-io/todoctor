use std::env;
use std::path::{Component, Path, PathBuf};

/// Returns `true` if `file` is not inside any of the `ignores` paths.
///
/// `file` comes from git and does not have to exist on disk: history also
/// has files that were renamed or deleted later. So both paths are made
/// relative to the working directory before they are compared, instead of
/// relying on `canonicalize`, which fails for missing files.
pub fn identify_not_ignored_file(file: &str, ignores: &[String]) -> bool {
    let cwd = env::current_dir().ok();
    let root = cwd.as_deref().and_then(|cwd| cwd.canonicalize().ok());
    let to_relative = |path: &str| {
        to_relative_path(Path::new(path), cwd.as_deref(), root.as_deref())
    };

    let file_path = to_relative(file);
    let is_ignored = ignores
        .iter()
        .any(|ignored| file_path.starts_with(to_relative(ignored)));

    !is_ignored
}

/// Makes `path` relative to the working directory. A path that exists is
/// resolved first, so `src`, `./src/` and an absolute path to `src` all give
/// `src`. A path that does not exist is only cleaned up.
fn to_relative_path(
    path: &Path,
    cwd: Option<&Path>,
    root: Option<&Path>,
) -> PathBuf {
    if let (Some(root), Ok(resolved)) = (root, path.canonicalize()) {
        if let Ok(relative) = resolved.strip_prefix(root) {
            return relative.to_path_buf();
        }
    }

    let path = cwd
        .and_then(|cwd| path.strip_prefix(cwd).ok())
        .unwrap_or(path);

    path.components()
        .filter(|component| !matches!(component, Component::CurDir))
        .collect()
}
