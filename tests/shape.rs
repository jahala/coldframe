//! The shape (docs/architecture.md, "Shape"): the core decides and the edge does the I/O.
//! `src/main.rs` and the files under `src/edge/` are the edge. Every other file under `src/`
//! is the core, and must not reach the disk, the clock, the environment, the process table,
//! the network or the terminal, nor call into the edge.

mod common;

use std::fs;

const IO_MARKERS: [&str; 15] = [
    "fs::",
    "process::",
    "env::",
    "net::",
    "io::stdin",
    "io::stdout",
    "io::stderr",
    "File::",
    "SystemTime",
    "Instant::now",
    "println!",
    "eprintln!",
    "print!",
    "eprint!",
    "crate::edge",
];

fn is_edge(path: &str) -> bool {
    path == "src/main.rs" || path.starts_with("src/edge/")
}

/// The I/O a core file reaches for, by marker. Lines that are only comments do not count.
fn io_in(text: &str) -> Vec<&'static str> {
    let code: String = text
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    IO_MARKERS
        .into_iter()
        .filter(|marker| code.contains(marker))
        .collect()
}

#[test]
fn the_detector_flags_io_and_passes_pure_code() {
    let io = "use std::{fs::read, env::var};\nfn f() { println!(\"x\"); }";
    assert_eq!(io_in(io), ["fs::", "env::", "println!"]);
    let pure =
        "/// Reads nothing: fs:: is named only in this comment.\npub fn add(a: u8) -> u8 { a + 1 }";
    assert_eq!(io_in(pure), Vec::<&str>::new());
    assert!(is_edge("src/main.rs") && is_edge("src/edge/disk.rs") && !is_edge("src/lib.rs"));
}

#[test]
fn the_core_does_no_io() {
    let core: Vec<String> = common::own_files()
        .into_iter()
        .filter(|path| path.starts_with("src/") && path.ends_with(".rs") && !is_edge(path))
        .collect();
    assert!(!core.is_empty(), "no core files under src/");
    let offenders: Vec<String> = core
        .into_iter()
        .filter_map(|path| {
            let found = io_in(&fs::read_to_string(&path).ok()?);
            (!found.is_empty()).then(|| format!("{path}: {found:?}"))
        })
        .collect();
    assert_eq!(offenders, Vec::<String>::new());
}
