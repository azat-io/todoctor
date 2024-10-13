use crate::exec::exec;
use std::error::Error;

pub async fn get_diff(commit_hash: &str) -> Result<String, Box<dyn Error>> {
    let git_diff_command = ["git", "diff", &format!("{}^!", commit_hash)];

    let output = exec(&git_diff_command).await?;

    Ok(output)
}
