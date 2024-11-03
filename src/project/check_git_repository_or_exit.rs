use crate::git::check_git_repository;
use std::process;

pub async fn check_git_repository_or_exit() {
    if !check_git_repository(".").await {
        eprintln!("Error: This is not a Git repository.");
        process::exit(1);
    }
}
