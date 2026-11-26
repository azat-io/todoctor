use crate::exec::exec_bytes;

pub async fn get_modified_files(
    previous_commit: &str,
    current_commit: &str,
) -> Vec<String> {
    let output = exec_bytes(&[
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
    // and the new path are returned. It also keeps every record at two
    // fields, which the pairing below relies on: without it, rename and copy
    // records ("R100\0<old>\0<new>\0") have three. Names that are not valid
    // UTF-8 are skipped instead of failing the whole list.
    output
        .split(|byte| *byte == 0)
        .filter(|part| !part.is_empty())
        .skip(1)
        .step_by(2)
        .filter_map(|file| std::str::from_utf8(file).ok())
        .map(|file| file.to_string())
        .collect()
}
