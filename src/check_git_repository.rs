use std::path::Path;
use tokio::fs;

pub async fn check_git_repository(dir: &str) -> bool {
    let git_path = Path::new(dir).join(".git");
    fs::metadata(git_path).await.is_ok()
}
