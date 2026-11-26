mod common;

use common::TempRepo;
use std::fs;
use todoctor::git::get_modified_files;

#[tokio::test]
async fn test_get_modified_files_with_renames_and_special_names() {
    let repo = TempRepo::new("modified-files").await;
    fs::write(repo.path().join("old.ts"), "// TODO: rename me\n").unwrap();
    fs::write(repo.path().join("plain.ts"), "// TODO: one\n").unwrap();
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "first"]);
    let first = repo.git(&["rev-parse", "HEAD"]).trim().to_string();

    repo.git(&["mv", "old.ts", "new.ts"]);
    fs::write(repo.path().join("plain.ts"), "// TODO: one\n// TODO: two\n")
        .unwrap();
    fs::write(repo.path().join("café.ts"), "// TODO: three\n").unwrap();
    fs::write(repo.path().join("my file.ts"), "// TODO: four\n").unwrap();
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "second"]);
    let second = repo.git(&["rev-parse", "HEAD"]).trim().to_string();

    let mut files = get_modified_files(&first, &second).await;
    files.sort();

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
