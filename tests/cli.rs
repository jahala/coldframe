//! The command's conventions (docs/architecture.md, "Exit codes"): it names its version, and a
//! usage error exits 2, prints nothing on standard output and writes nothing.

mod common;

use std::fs;
use std::process::{Command, Output};

fn coldframe(args: &[&str], cwd: &std::path::Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_coldframe"))
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("coldframe runs")
}

#[test]
fn it_names_its_version() {
    let scratch = common::Scratch::new("cli");
    let out = coldframe(&["--version"], &scratch.path);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&out.stdout), "coldframe 0.1.0\n");
    assert!(out.stderr.is_empty());
}

#[test]
fn help_prints_the_usage_and_exits_0() {
    let scratch = common::Scratch::new("cli");
    let out = coldframe(&["--help"], &scratch.path);
    assert_eq!(out.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&out.stdout).starts_with("usage: coldframe"));
}

#[test]
fn a_usage_error_exits_2_and_writes_nothing() {
    for args in [&[][..], &["frobnicate"][..], &["--version", "extra"][..]] {
        let scratch = common::Scratch::new("cli");
        let out = coldframe(args, &scratch.path);
        assert_eq!(out.status.code(), Some(2), "{args:?}");
        assert!(out.stdout.is_empty(), "{args:?}");
        assert!(String::from_utf8_lossy(&out.stderr).contains("usage: coldframe"));
        let left: Vec<_> = fs::read_dir(&scratch.path).expect("read scratch").collect();
        assert!(left.is_empty(), "{args:?} wrote into its folder");
    }
}
