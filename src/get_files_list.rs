use crate::exec::exec;

pub async fn get_files_list(
    commit: Option<&str>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let git_command = if let Some(commit) = commit {
        vec!["git", "ls-tree", "-r", "--name-only", commit]
    } else {
        vec!["git", "ls-files"]
    };

    let output = exec(&git_command).await?;
    let files: Vec<String> =
        output.lines().map(|line| line.to_string()).collect();
    Ok(files)
}
