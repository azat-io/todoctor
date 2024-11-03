use tokio::fs;

pub async fn make_dir(output_directory: &str) {
    if fs::metadata(output_directory).await.is_ok() {
        if let Err(e) = fs::remove_dir_all(output_directory).await {
            eprintln!(
                "Error removing output directory {}: {:?}",
                output_directory, e
            );
            return;
        }
    }

    if let Err(e) = fs::create_dir_all(output_directory).await {
        eprintln!(
            "Error creating output directory {}: {:?}",
            output_directory, e
        );
        return;
    }
}
