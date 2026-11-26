use crate::exec::exec;

pub async fn get_modified_files(
    previous_commit: &str,
    current_commit: &str,
) -> Vec<String> {
    let output = exec(&[
        "git",
        "diff",
        "--name-status",
        "--no-renames",
        "-z",
        previous_commit,
        current_commit,
    ])
    .await
    .expect("Failed to get modified files");

    // With -z the output is "<status>\0<path>\0" for each file. Paths are
    // not quoted, so names with spaces or non-ASCII characters stay intact.
    // --no-renames reports a rename as a delete plus an add, so both the old
    // and the new path are returned.
    output
        .split('\0')
        .filter(|part| !part.is_empty())
        .skip(1)
        .step_by(2)
        .map(|file| file.to_string())
        .collect()
}
