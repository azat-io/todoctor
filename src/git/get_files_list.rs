use crate::exec::exec_bytes;

pub async fn get_files_list(
    commit: Option<&str>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let git_command = if let Some(commit) = commit {
        vec!["git", "ls-tree", "-r", "-z", "--name-only", commit]
    } else {
        vec!["git", "ls-files", "-z"]
    };

    let output = exec_bytes(&git_command).await?;
    // Names that are not valid UTF-8 cannot be read as source files here, so
    // they are skipped instead of failing the whole list.
    let files: Vec<String> = output
        .split(|byte| *byte == 0)
        .filter(|file| !file.is_empty())
        .filter_map(|file| std::str::from_utf8(file).ok())
        .map(|file| file.to_string())
        .collect();
    Ok(files)
}
