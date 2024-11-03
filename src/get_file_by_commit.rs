use crate::exec::exec;

pub async fn get_file_by_commit(
    commit: &str,
    file: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let output =
        exec(&["git", "show", &format!("{}:{}", commit, file)]).await?;
    Ok(output)
}
