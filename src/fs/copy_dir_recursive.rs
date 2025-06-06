use futures::future::BoxFuture;
use std::path::Path;
use tokio::{fs, io};

pub async fn copy_dir_recursive(
    src: &Path,
    output_dir: &Path,
) -> io::Result<()> {
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).await?;
    }

    let mut entries = fs::read_dir(src).await?;
    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        let entry_name = entry.file_name();
        let dest_path = output_dir.join(entry_name);

        if entry_path.is_dir() {
            copy_dir_recursive_boxed(&entry_path, &dest_path).await?;
        } else {
            fs::copy(&entry_path, &dest_path).await?;
        }
    }

    Ok(())
}

fn copy_dir_recursive_boxed<'a>(
    src: &'a Path,
    dst: &'a Path,
) -> BoxFuture<'a, io::Result<()>> {
    Box::pin(copy_dir_recursive(src, dst))
}
