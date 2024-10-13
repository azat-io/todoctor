use std::error::Error;
use tokio::process::Command;

pub async fn exec(command: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new(command[0])
        .args(&command[1..])
        .output()
        .await?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        Ok(stdout)
    } else {
        let stderr = String::from_utf8(output.stderr)?;
        Err(format!("Command failed: {}", stderr).into())
    }
}
