use crate::exec::exec;

pub async fn get_modified_files(
    previous_commit: &str,
    current_commit: &str,
) -> Vec<String> {
    let output = exec(&[
        "git",
        "diff",
        "--name-status",
        previous_commit,
        current_commit,
    ])
    .await
    .expect("Failed to get modified files");

    output
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                parts[1].to_string()
            } else {
                "".to_string()
            }
        })
        .filter(|s| !s.is_empty())
        .collect()
}
