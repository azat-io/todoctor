use todoctor::comments::identify_not_ignored_file;

// Cargo runs integration tests from the package root, so `src` and `tests`
// exist there.

fn ignores(paths: &[&str]) -> Vec<String> {
    paths.iter().map(|path| path.to_string()).collect()
}

#[test]
fn test_identify_not_ignored_file() {
    let ignored = ignores(&["src"]);
    assert!(!identify_not_ignored_file("src/main.rs", &ignored));
    assert!(identify_not_ignored_file("tests/exec.rs", &ignored));
}

#[test]
fn test_identify_not_ignored_file_missing_on_disk() {
    // History has files that were renamed or deleted later, so they are not
    // on disk any more. They must still be matched against the ignores.
    let ignored = ignores(&["src"]);
    assert!(!identify_not_ignored_file("src/renamed.rs", &ignored));
    assert!(identify_not_ignored_file("src-old/renamed.rs", &ignored));

    let ignored = ignores(&["./removed/"]);
    assert!(!identify_not_ignored_file("removed/file.rs", &ignored));
}

#[test]
fn test_identify_not_ignored_file_with_ignore_path_forms() {
    let absolute = std::env::current_dir().unwrap().join("src");
    for ignored in ["./src", "src/", absolute.to_str().unwrap()] {
        let ignored = ignores(&[ignored]);
        assert!(!identify_not_ignored_file("src/main.rs", &ignored));
        assert!(!identify_not_ignored_file("src/renamed.rs", &ignored));
        assert!(identify_not_ignored_file("tests/exec.rs", &ignored));
    }
}
