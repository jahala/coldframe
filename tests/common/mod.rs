//! What the tests share: a temporary folder each test makes for itself, and the files this
//! repository wrote.
#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT: AtomicUsize = AtomicUsize::new(0);

/// A temporary folder this test creates under TMPDIR and removes when it is dropped.
/// `scripts/test.sh` points TMPDIR at a folder of its own and fails the run if anything is left.
pub struct Scratch {
    pub path: PathBuf,
}

impl Scratch {
    pub fn new(name: &str) -> Scratch {
        let n = NEXT.fetch_add(1, Ordering::SeqCst);
        let path =
            std::env::temp_dir().join(format!("coldframe-{name}-{}-{n}", std::process::id()));
        fs::create_dir_all(&path).expect("make the scratch folder");
        Scratch { path }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        // The one removal a run may make: the temporary folder it created itself.
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// Every file this repository wrote: tracked, or new and not ignored.
pub fn own_files() -> Vec<String> {
    let out = Command::new("git")
        .args([
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ])
        .output()
        .expect("git runs");
    assert!(out.status.success(), "git ls-files failed");
    String::from_utf8(out.stdout)
        .expect("git lists paths as UTF-8")
        .split('\0')
        .filter(|path| !path.is_empty() && std::path::Path::new(path).is_file())
        .map(str::to_string)
        .collect()
}
