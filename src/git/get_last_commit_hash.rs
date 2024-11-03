use crate::exec::exec;

pub async fn get_last_commit_hash() -> Option<String> {
    match exec(&["git", "rev-parse", "HEAD"]).await {
        Ok(output) => Some(output.trim().to_string()),
        Err(_) => None,
    }
}
