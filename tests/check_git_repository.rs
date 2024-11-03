use std::path::Path;
use todoctor::git::check_git_repository;

#[tokio::test]
async fn test_is_git_repository_true() {
    let current_file = file!();
    let current_dir = Path::new(current_file).parent().unwrap();
    let parent_dir = current_dir.parent().unwrap();
    let parent_dir_str = parent_dir.to_str().unwrap();
    let result = check_git_repository(parent_dir_str).await;
    assert!(result);
}

#[tokio::test]
async fn test_is_git_repository_false() {
    let current_file = file!();
    let current_dir = Path::new(current_file).parent().unwrap();
    let current_dir_str = current_dir.to_str().unwrap();

    let result = check_git_repository(current_dir_str).await;
    assert!(!result);
}
