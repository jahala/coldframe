//! Hard limits a file can break (AGENTS.md, "Hard limits"). Each detector is a function, and
//! each one is shown a sample it must flag, so a check that reads text proves it still finds
//! what it looks for.

mod common;

use std::fs;

/// Placeholder and skip markers, built from pieces so this file does not carry them itself.
fn placeholder_markers() -> [String; 5] {
    [
        ["TO", "DO"].concat(),
        ["FIX", "ME"].concat(),
        ["todo", "!("].concat(),
        ["unimplemented", "!("].concat(),
        ["#[", "ignore"].concat(),
    ]
}

/// tend2's renderer, copied beside the map as tend2 writes it. Its prose is tend2's.
const VENDORED: [&str; 2] = ["docs/tend2/loop.css", "docs/tend2/loop.js"];

fn em_dashes(text: &str) -> bool {
    text.contains('\u{2014}')
}

/// Deletion calls. coldframe's own code deletes nothing yet; when prune and the run scope
/// arrive, they move files to the Trash through one function, and this list names it.
fn deletion_calls(text: &str) -> Vec<&'static str> {
    [
        "remove_file",
        "remove_dir",
        "\"rm\"",
        "\"rmdir\"",
        "\"unlink\"",
    ]
    .into_iter()
    .filter(|call| text.contains(call))
    .collect()
}

fn placeholders(text: &str) -> Vec<String> {
    placeholder_markers()
        .into_iter()
        .filter(|marker| text.contains(marker.as_str()))
        .collect()
}

/// A dependency whose name says it is a mock. Tests use real values, real folders and real
/// processes (AGENTS.md, "Shape").
fn mock_dependencies(cargo_toml: &str) -> Vec<String> {
    cargo_toml
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, _)| key.trim().to_string())
        .filter(|key| key.to_ascii_lowercase().contains("mock"))
        .collect()
}

fn is_code(path: &str) -> bool {
    [".rs", ".sh", ".yml", ".yaml", ".toml"]
        .iter()
        .any(|kind| path.ends_with(kind))
}

#[test]
fn every_detector_flags_its_sample() {
    assert!(em_dashes("a \u{2014} b"));
    assert!(!em_dashes("a - b"));
    assert_eq!(
        deletion_calls("std::fs::remove_dir_all(p); Command::new(\"rm\")"),
        ["remove_dir", "\"rm\""]
    );
    assert_eq!(deletion_calls("trash(p)"), Vec::<&str>::new());
    assert_eq!(
        placeholders(&format!("// {}: later", ["TO", "DO"].concat())).len(),
        1
    );
    assert_eq!(
        placeholders(&format!("{}]\nfn t() {{}}", ["#[", "ignore"].concat())).len(),
        1
    );
    assert_eq!(placeholders("fn done() {}"), Vec::<String>::new());
    assert_eq!(
        mock_dependencies("[dev-dependencies]\nmockall = \"0.13\"\nserde = \"1\""),
        ["mockall"]
    );
}

#[test]
fn no_file_this_repository_wrote_carries_an_em_dash() {
    let files = common::own_files();
    assert!(!files.is_empty(), "git listed no files");
    let offenders: Vec<&String> = files
        .iter()
        .filter(|path| !VENDORED.contains(&path.as_str()))
        .filter(|path| fs::read(path).is_ok_and(|b| em_dashes(&String::from_utf8_lossy(&b))))
        .collect();
    assert_eq!(offenders, Vec::<&String>::new());
}

#[test]
fn coldframe_s_own_code_deletes_nothing() {
    let offenders: Vec<String> = common::own_files()
        .into_iter()
        .filter(|path| path.starts_with("src/"))
        .filter_map(|path| {
            let calls = deletion_calls(&fs::read_to_string(&path).ok()?);
            (!calls.is_empty()).then(|| format!("{path}: {calls:?}"))
        })
        .collect();
    assert_eq!(offenders, Vec::<String>::new());
}

#[test]
fn no_code_carries_a_placeholder_or_a_skipped_test() {
    let offenders: Vec<String> = common::own_files()
        .into_iter()
        .filter(|path| is_code(path))
        .filter_map(|path| {
            let found = placeholders(&fs::read_to_string(&path).ok()?);
            (!found.is_empty()).then(|| format!("{path}: {found:?}"))
        })
        .collect();
    assert_eq!(offenders, Vec::<String>::new());
}

#[test]
fn no_dependency_is_a_mock() {
    let cargo = fs::read_to_string("Cargo.toml").expect("Cargo.toml is missing");
    assert_eq!(mock_dependencies(&cargo), Vec::<String>::new());
}
