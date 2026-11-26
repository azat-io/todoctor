use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use todoctor::git::get_files_list;

fn git(dir: &PathBuf, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(["-c", "user.name=Test", "-c", "user.email=test@example.com"])
        .args(args)
        .current_dir(dir)
        .output()
        .expect("Failed to run git");
    assert!(output.status.success(), "git {:?} failed", args);
    String::from_utf8(output.stdout).unwrap()
}

#[tokio::test]
async fn test_get_files_list_with_special_file_names() {
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
