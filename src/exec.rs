use std::error::Error;
use tokio::process::Command;

pub async fn exec(command: &[&str]) -> Result<String, Box<dyn Error>> {
    let stdout = exec_bytes(command).await?;
    Ok(String::from_utf8(stdout)?)
}

/// Runs the command and returns stdout as raw bytes. Use it when the output
/// may hold bytes that are not valid UTF-8, such as `git ... -z` file lists.
pub async fn exec_bytes(command: &[&str]) -> Result<Vec<u8>, Box<dyn Error>> {
    let output = Command::new(command[0])
        .args(&command[1..])
        .output()
        .await?;

    if output.status.success() {
        Ok(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Command failed: {}", stderr).into())
    }
}
