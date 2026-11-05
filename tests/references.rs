//! The references every bed keeps (the garden's law, fit check F7): the core page within 150
//! lines, a one-line instruction file that imports it, the two documents, and every repository
//! path they cite resolving in the tree.

use std::fs;
use std::path::Path;

const DOCUMENTS: [&str; 4] = [
    "AGENTS.md",
    "docs/architecture.md",
    "docs/engineering.md",
    "docs/decisions.md",
];
const FILE_KINDS: [&str; 11] = [
    "md", "rs", "toml", "json", "yml", "yaml", "html", "sh", "css", "js", "lock",
];

/// The repository paths a document cites: backticked spans shaped like a path in this tree.
/// Another repository's path is written `owner/repo:path`, and the colon keeps it out. A span
/// with a placeholder such as `<id>` or a home folder such as `~/x` is not a path in the tree.
fn cited_paths(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for (index, span) in text.split('`').enumerate() {
        if index % 2 == 0 || span.contains('\n') || span.is_empty() {
            continue;
        }
        let shaped = span.trim_end_matches('/').split('/').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "_.-".contains(c))
        });
        let file_like = span.contains('/')
            || span
                .rsplit_once('.')
                .is_some_and(|(stem, kind)| !stem.is_empty() && FILE_KINDS.contains(&kind));
        if shaped && file_like && span != "." && span != ".." {
            found.push(span.to_string());
        }
    }
    found
}

#[test]
fn the_core_page_holds_at_most_150_lines() {
    let text = fs::read_to_string("AGENTS.md").expect("AGENTS.md is missing");
    let lines = text.lines().count();
    assert!(lines <= 150, "AGENTS.md has {lines} lines");
}

#[test]
fn the_instruction_file_is_one_line_that_imports_the_core_page() {
    assert_eq!(
        fs::read_to_string("CLAUDE.md").expect("CLAUDE.md is missing"),
        "@AGENTS.md\n"
    );
}

#[test]
fn the_core_page_and_the_documents_exist() {
    for doc in DOCUMENTS {
        assert!(Path::new(doc).is_file(), "{doc} is missing");
    }
}

#[test]
fn every_path_the_documents_cite_resolves_in_the_tree() {
    let mut missing = Vec::new();
    for doc in DOCUMENTS {
        let text = fs::read_to_string(doc).unwrap_or_default();
        for path in cited_paths(&text) {
            if !Path::new(&path).exists() {
                missing.push(format!("{doc} cites {path}"));
            }
        }
    }
    assert_eq!(missing, Vec::<String>::new());
}

#[test]
fn a_cited_path_is_a_backticked_repository_path_never_another_repositorys() {
    let text = [
        "Read `AGENTS.md`, then `docs/architecture.md` and `src/`.",
        "Not paths: `trash`, `cargo test`, `## Tried`, `kind: [\"repository\"]`, `~/.config/coldframe/config.toml`,",
        "`jahala/plotplot:docs/ideas/coldframe.md`, `docs/preflight/<slice>.md`, `.rs`, `.`.",
    ]
    .join("\n");
    assert_eq!(
        cited_paths(&text),
        ["AGENTS.md", "docs/architecture.md", "src/"]
    );
}
