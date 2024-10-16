use crate::exec::exec;

pub async fn get_history(months: Option<u32>) -> Vec<(String, String)> {
    let since_period = months
        .map(|m| format!("{} months ago", m))
        .unwrap_or_else(|| "3 month ago".to_string());

    let git_log_command = [
        "git",
        "log",
        &format!("--since={}", since_period),
        "--pretty=format:%H %ad",
        "--date=short",
    ];

    let output = match exec(&git_log_command).await {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to execute git log: {:?}", e);
            return Vec::new();
        }
    };

    let mut commits = Vec::new();

    for line in output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let commit_hash = parts[0].to_string();
            let date = parts[1].to_string();
            commits.push((commit_hash, date));
        }
    }

    commits
}
