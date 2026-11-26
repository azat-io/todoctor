use crate::exec::exec;

pub async fn get_files_list(
    commit: Option<&str>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let git_command = if let Some(commit) = commit {
        vec!["git", "ls-tree", "-r", "-z", "--name-only", commit]
    } else {
        vec!["git", "ls-files", "-z"]
    };

    let output = exec(&git_command).await?;
    let files: Vec<String> = output
        .split('\0')
        .filter(|file| !file.is_empty())
        .map(|file| file.to_string())
        .collect();
    Ok(files)
}
