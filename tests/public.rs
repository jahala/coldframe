//! What a public repository must not carry (docs/engineering.md, "Rules and their guards"):
//! a path on the owner's machine, or the name of one of the owner's private projects. The
//! garden's own tools may be named, as the public umbrella names them. The private names are
//! kept here only as hashes, so this file does not publish what it guards.

mod common;

use std::fs;

/// FNV-1a, 64 bits: small, stable across Rust releases, and enough to recognise a word.
fn fnv1a(word: &str) -> u64 {
    word.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// The owner's private projects, hashed from their lowercase names.
const PRIVATE_NAMES: [u64; 8] = [
    0xf284_a59e_8900_a2fc,
    0x0fb1_a692_ffc8_e976,
    0x28c0_fe54_fbca_f424,
    0x99ea_79eb_b630_970e,
    0x4ccd_b363_9430_bb7e,
    0x31b6_3ab9_0090_8604,
    0x77ea_e572_ed45_b463,
    0x02f1_0b75_5d78_3b92,
];

/// Folders that only exist on someone's machine, built from pieces so this file passes.
fn machine_paths() -> [String; 3] {
    [
        ["/Us", "ers/"].concat(),
        ["/ho", "me/"].concat(),
        ["/var/", "folders/"].concat(),
    ]
}

/// The private names a text carries: each word, split on anything that is not a letter, a
/// digit or a hyphen, compared by hash.
fn private_names(text: &str, hashes: &[u64]) -> Vec<String> {
    let lower = text.to_lowercase();
    let mut found: Vec<String> = lower
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
        .filter(|word| !word.is_empty() && hashes.contains(&fnv1a(word)))
        .map(str::to_string)
        .collect();
    found.sort();
    found.dedup();
    found
}

fn personal_paths(text: &str) -> Vec<String> {
    machine_paths()
        .into_iter()
        .filter(|path| text.contains(path.as_str()))
        .collect()
}

#[test]
fn the_detectors_flag_their_samples() {
    let sample = [fnv1a("sample-private-name")];
    assert_eq!(
        private_names(
            "Ask Sample-Private-Name, then sample-private-name.com.",
            &sample
        ),
        ["sample-private-name"]
    );
    assert_eq!(
        private_names("graft and mull", &sample),
        Vec::<String>::new()
    );
    let home = ["/Us", "ers/someone/x"].concat();
    assert_eq!(
        personal_paths(&format!("see {home}")),
        [["/Us", "ers/"].concat()]
    );
    assert_eq!(
        personal_paths("see ~/actions-runner-x"),
        Vec::<String>::new()
    );
}

#[test]
fn no_file_carries_a_machine_path_or_a_private_project_name() {
    let mut offenders = Vec::new();
    for path in common::own_files() {
        let Ok(bytes) = fs::read(&path) else { continue };
        let text = String::from_utf8_lossy(&bytes);
        let found: Vec<String> = personal_paths(&text)
            .into_iter()
            .chain(
                private_names(&text, &PRIVATE_NAMES)
                    .into_iter()
                    .map(|_| "a private name".to_string()),
            )
            .collect();
        if !found.is_empty() {
            offenders.push(format!("{path}: {found:?}"));
        }
    }
    assert_eq!(offenders, Vec::<String>::new());
}
