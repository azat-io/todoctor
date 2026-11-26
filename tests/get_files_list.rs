use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use todoctor::git::get_files_list;
use tokio::sync::Mutex;

// Both tests change the working directory of the process, so they must not
// run at the same time.
static CWD_LOCK: Mutex<()> = Mutex::const_new(());

fn git(dir: &PathBuf, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(["-c", "user.name=Test", "-c", "user.email=test@example.com"])
        .args(args)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_OBJECT_DIRECTORY")
        .current_dir(dir)
        .output()
        .expect("Failed to run git");
    assert!(output.status.success(), "git {:?} failed", args);
    String::from_utf8(output.stdout).unwrap()
}

#[tokio::test]
async fn test_get_files_list_with_special_file_names() {
    let _guard = CWD_LOCK.lock().await;
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "todoctor-files-list-{}-{}",
        std::process::id(),
        nanos
    ));
    fs::create_dir_all(&dir).unwrap();

    git(&dir, &["init", "-q"]);
    fs::write(dir.join("café.ts"), "// TODO: one\n").unwrap();
    fs::write(dir.join("my file.ts"), "// TODO: two\n").unwrap();
    fs::write(dir.join("a\"b.ts"), "// TODO: three\n").unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-q", "-m", "init"]);
    let commit = git(&dir, &["rev-parse", "HEAD"]).trim().to_string();

    std::env::set_current_dir(&dir).unwrap();

    let expected = vec![
        "a\"b.ts".to_string(),
        "café.ts".to_string(),
        "my file.ts".to_string(),
    ];

    let mut files = get_files_list(None).await.unwrap();
    files.sort();
    let mut files_by_commit = get_files_list(Some(&commit)).await.unwrap();
    files_by_commit.sort();

    std::env::set_current_dir(std::env::temp_dir()).unwrap();
    fs::remove_dir_all(&dir).unwrap();

    assert_eq!(files, expected);
    assert_eq!(files_by_commit, expected);
}

#[cfg(unix)]
#[tokio::test]
async fn test_get_files_list_skips_non_utf8_file_names() {
    let _guard = CWD_LOCK.lock().await;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "todoctor-files-list-bytes-{}-{}",
        std::process::id(),
        nanos
    ));
    fs::create_dir_all(&dir).unwrap();

    git(&dir, &["init", "-q"]);
    fs::write(dir.join("ok.ts"), "// TODO: one\n").unwrap();
    git(&dir, &["add", "."]);
    // Some file systems refuse names that are not UTF-8, so the entry is
    // added to the index directly instead of being written to disk.
    let blob = git(&dir, &["hash-object", "-w", "ok.ts"])
        .trim()
        .to_string();
    let status = Command::new("git")
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_OBJECT_DIRECTORY")
        .args(["update-index", "--add", "--cacheinfo", "100644"])
        .arg(&blob)
        .arg(OsStr::from_bytes(b"bad\xff.ts"))
        .current_dir(&dir)
        .status()
        .expect("Failed to run git");
    assert!(status.success());
    git(&dir, &["commit", "-q", "-m", "init"]);
    let commit = git(&dir, &["rev-parse", "HEAD"]).trim().to_string();

    std::env::set_current_dir(&dir).unwrap();

    let files = get_files_list(None).await;
    let files_by_commit = get_files_list(Some(&commit)).await;

    std::env::set_current_dir(std::env::temp_dir()).unwrap();
    fs::remove_dir_all(&dir).unwrap();

    assert_eq!(files.unwrap(), vec!["ok.ts".to_string()]);
    assert_eq!(files_by_commit.unwrap(), vec!["ok.ts".to_string()]);
}
