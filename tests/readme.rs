//! The README's example runs: each `$ coldframe ...` line in a fenced block is run, and its
//! output must be the lines that follow it, up to the next prompt or the end of the block.

use std::fs;
use std::process::Command;

/// Each example: the arguments after `$ coldframe`, and the output the README shows.
fn examples(readme: &str) -> Vec<(Vec<String>, String)> {
    let mut found = Vec::new();
    let mut in_block = false;
    let mut current: Option<(Vec<String>, String)> = None;
    for line in readme.lines() {
        if line.starts_with("```") {
            in_block = !in_block;
            found.extend(current.take());
            continue;
        }
        if !in_block {
            continue;
        }
        if let Some(rest) = line.strip_prefix("$ coldframe") {
            found.extend(current.take());
            let args = rest.split_whitespace().map(str::to_string).collect();
            current = Some((args, String::new()));
        } else if let Some((_, out)) = current.as_mut() {
            out.push_str(line);
            out.push('\n');
        }
    }
    found
}

#[test]
fn the_reader_finds_each_example_and_its_output() {
    let readme = "Text\n```\n$ coldframe --version\ncoldframe 9.9.9\n$ coldframe --help\nusage\n```\n```bash\ncargo build\n```\n";
    assert_eq!(
        examples(readme),
        [
            (
                vec!["--version".to_string()],
                "coldframe 9.9.9\n".to_string()
            ),
            (vec!["--help".to_string()], "usage\n".to_string()),
        ]
    );
}

#[test]
fn every_example_in_the_readme_prints_what_it_shows() {
    let readme = fs::read_to_string("README.md").expect("README.md is missing");
    let found = examples(&readme);
    assert!(!found.is_empty(), "the README shows no example");
    for (args, shown) in found {
        let out = Command::new(env!("CARGO_BIN_EXE_coldframe"))
            .args(&args)
            .output()
            .expect("coldframe runs");
        assert_eq!(
            String::from_utf8_lossy(&out.stdout),
            shown,
            "coldframe {args:?}"
        );
    }
}
