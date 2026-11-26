use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs};
use tokio::sync::{Mutex, MutexGuard};

// The repository becomes the working directory of the whole process, so only
// one test may use one at a time.
static CWD_LOCK: Mutex<()> = Mutex::const_new(());

/// A git repository in a temp directory. While it is alive, it is the working
/// directory of the process. When it is dropped, even after a failed assert,
/// the previous working directory is restored and the repository is removed.
pub struct TempRepo {
    path: PathBuf,
    previous_dir: PathBuf,
    _lock: MutexGuard<'static, ()>,
}

impl TempRepo {
    pub async fn new(name: &str) -> Self {
        let lock = CWD_LOCK.lock().await;
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = env::temp_dir().join(format!(
            "todoctor-{}-{}-{}",
            name,
            process::id(),
            nanos
        ));
        fs::create_dir_all(&path).unwrap();

        let repo = TempRepo {
            previous_dir: env::current_dir().unwrap(),
            path,
            _lock: lock,
        };
        repo.git(&["init", "-q"]);
        env::set_current_dir(&repo.path).unwrap();
        repo
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Runs git in the repository without the user's global and system
    /// config, so settings such as `commit.gpgsign` or hooks cannot change
    /// or break the fixture.
    pub fn git<S: AsRef<OsStr>>(&self, args: &[S]) -> String {
        let output = Command::new("git")
            .args(["-c", "user.name=Test", "-c", "user.email=test@example.com"])
            .args(["-c", "commit.gpgsign=false"])
            .args(args)
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .env_remove("GIT_OBJECT_DIRECTORY")
            .current_dir(&self.path)
            .output()
            .expect("Failed to run git");
        let args: Vec<&OsStr> = args.iter().map(|arg| arg.as_ref()).collect();
        assert!(
            output.status.success(),
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = env::set_current_dir(&self.previous_dir);
        let _ = fs::remove_dir_all(&self.path);
    }
}
