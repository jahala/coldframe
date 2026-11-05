//! The CI rules this repository's own workflows keep (AGENTS.md, "How work lands"): every job
//! runs on the owner's runners with the way back to hosted runners, every job's first step
//! points TMPDIR at the runner's temp folder, and every action is pinned to a full commit.

use std::fs;

const RUNS_ON: &str =
    r#"runs-on: ${{ fromJSON(vars.CI_RUNS_ON || '["self-hosted", "macOS", "coldframe"]') }}"#;
const TMPDIR_STEP: &str = r#"- run: echo "TMPDIR=$RUNNER_TEMP" >> "$GITHUB_ENV""#;

/// What a workflow file breaks, one line per finding. It reads lines, not YAML: each
/// `runs-on:` line must be the convention, the first line after each `steps:` that is not a
/// comment must be the TMPDIR step, and each `uses:` must name a 40-character commit.
fn findings(workflow: &str) -> Vec<String> {
    let lines: Vec<&str> = workflow.lines().map(str::trim).collect();
    let mut found = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let n = i + 1;
        if line.starts_with("runs-on:") && *line != RUNS_ON {
            found.push(format!("line {n}: runs-on is not the convention"));
        }
        if *line == "steps:" {
            let first = lines[i + 1..]
                .iter()
                .find(|l| !l.is_empty() && !l.starts_with('#'));
            if first != Some(&TMPDIR_STEP) {
                found.push(format!("line {n}: the first step is not the TMPDIR step"));
            }
        }
        if let Some(rest) = line
            .strip_prefix("- uses: ")
            .or(line.strip_prefix("uses: "))
        {
            let pinned = rest.split_once('@').is_some_and(|(_, at)| {
                let sha = at.split_whitespace().next().unwrap_or("");
                sha.len() == 40 && sha.chars().all(|c| c.is_ascii_hexdigit())
            });
            if !pinned {
                found.push(format!("line {n}: an action is not pinned to a commit"));
            }
        }
    }
    found
}

#[test]
fn the_reader_flags_each_broken_rule_and_passes_a_kept_one() {
    let kept = format!(
        "jobs:\n  check:\n    {RUNS_ON}\n    steps:\n      # first\n      {TMPDIR_STEP}\n      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n"
    );
    assert_eq!(findings(&kept), Vec::<String>::new());
    let broken = "jobs:\n  check:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n";
    assert_eq!(
        findings(broken),
        [
            "line 3: runs-on is not the convention",
            "line 4: the first step is not the TMPDIR step",
            "line 5: an action is not pinned to a commit",
        ]
    );
}

#[test]
fn every_workflow_keeps_the_rules() {
    let dir = fs::read_dir(".github/workflows").expect(".github/workflows is missing");
    let mut checked = 0;
    let mut all = Vec::new();
    for entry in dir {
        let path = entry.expect("read the workflows folder").path();
        let text = fs::read_to_string(&path).expect("read a workflow");
        checked += 1;
        for finding in findings(&text) {
            all.push(format!("{}: {finding}", path.display()));
        }
    }
    assert!(checked > 0, "no workflow files");
    assert_eq!(all, Vec::<String>::new());
}
