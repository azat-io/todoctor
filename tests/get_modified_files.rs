use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use todoctor::git::get_modified_files;

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
async fn test_get_modified_files_with_renames_and_special_names() {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "todoctor-modified-files-{}-{}",
        std::process::id(),
        nanos
    ));
    fs::create_dir_all(&dir).unwrap();

    git(&dir, &["init", "-q"]);
    fs::write(dir.join("old.ts"), "// TODO: rename me\n").unwrap();
    fs::write(dir.join("plain.ts"), "// TODO: one\n").unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-q", "-m", "first"]);
    let first = git(&dir, &["rev-parse", "HEAD"]).trim().to_string();

    git(&dir, &["mv", "old.ts", "new.ts"]);
    fs::write(dir.join("plain.ts"), "// TODO: one\n// TODO: two\n").unwrap();
    fs::write(dir.join("café.ts"), "// TODO: three\n").unwrap();
    fs::write(dir.join("my file.ts"), "// TODO: four\n").unwrap();
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "-q", "-m", "second"]);
    let second = git(&dir, &["rev-parse", "HEAD"]).trim().to_string();

    std::env::set_current_dir(&dir).unwrap();
    let mut files = get_modified_files(&first, &second).await;
    files.sort();
    std::env::set_current_dir(std::env::temp_dir()).unwrap();
    fs::remove_dir_all(&dir).unwrap();

    assert_eq!(
        files,
        vec![
            "café.ts".to_string(),
            "my file.ts".to_string(),
            "new.ts".to_string(),
            "old.ts".to_string(),
            "plain.ts".to_string(),
        ]
    );
}
