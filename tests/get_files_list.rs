mod common;

use common::TempRepo;
use std::fs;
use todoctor::git::get_files_list;

#[tokio::test]
async fn test_get_files_list_with_special_file_names() {
    let repo = TempRepo::new("files-list").await;
    fs::write(repo.path().join("café.ts"), "// TODO: one\n").unwrap();
    fs::write(repo.path().join("my file.ts"), "// TODO: two\n").unwrap();
    // Windows does not allow `"` in file names.
    #[cfg(unix)]
    fs::write(repo.path().join("a\"b.ts"), "// TODO: three\n").unwrap();
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "init"]);
    let commit = repo.git(&["rev-parse", "HEAD"]).trim().to_string();

    let mut expected = vec!["café.ts".to_string(), "my file.ts".to_string()];
    if cfg!(unix) {
        expected.insert(0, "a\"b.ts".to_string());
    }

    let mut files = get_files_list(None).await.unwrap();
    files.sort();
    let mut files_by_commit = get_files_list(Some(&commit)).await.unwrap();
    files_by_commit.sort();

    assert_eq!(files, expected);
    assert_eq!(files_by_commit, expected);
}

#[cfg(unix)]
#[tokio::test]
async fn test_get_files_list_skips_non_utf8_file_names() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let repo = TempRepo::new("files-list-bytes").await;
    fs::write(repo.path().join("ok.ts"), "// TODO: one\n").unwrap();
    repo.git(&["add", "."]);
    // Some file systems refuse names that are not UTF-8, so the entry is
    // added to the index directly instead of being written to disk.
    let blob = repo.git(&["hash-object", "-w", "ok.ts"]).trim().to_string();
    repo.git(&[
        OsStr::new("update-index"),
        OsStr::new("--add"),
        OsStr::new("--cacheinfo"),
        OsStr::new("100644"),
        OsStr::new(&blob),
        OsStr::from_bytes(b"bad\xff.ts"),
    ]);
    repo.git(&["commit", "-q", "-m", "init"]);
    let commit = repo.git(&["rev-parse", "HEAD"]).trim().to_string();

    let files = get_files_list(None).await.unwrap();
    let files_by_commit = get_files_list(Some(&commit)).await.unwrap();

    assert_eq!(files, vec!["ok.ts".to_string()]);
    assert_eq!(files_by_commit, vec!["ok.ts".to_string()]);
}
