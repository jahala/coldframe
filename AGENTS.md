# coldframe

coldframe is the bench where a change's tests run before it goes out. It owns how tests run on the owner's own machines: the self-hosted runners and the load they put on the machine, each project's declared test tiers and their measured times, and the hygiene of each run (a run leaves nothing behind, and holds on a loaded machine). It is one tool in the plotplot garden (its umbrella is the jahala/plotplot repository), in its Infrastructure row.

Read first, in this order:
1. This page.
2. `HANDOVER.md`: the brief, the first milestone and the build order.
3. `docs/build-plan.md`: the build order cut into slices, one pull request each.
4. `docs/architecture.md` (the shape, exit codes and data lifetimes), `docs/engineering.md` (the testing doctrine, and which check holds each rule) and `docs/decisions.md`.
5. The design: `jahala/plotplot:docs/ideas/coldframe.md`.
6. The garden's law: `jahala/plotplot:CLAUDE.md` and `jahala/plotplot:docs/building-the-garden.md`.
7. The evidence the design starts from, in the private notes repository: `jahala/coldframe-notes:docs/research/`.

## What coldframe is not

- **Not a judge of test changes.** Deleted, skipped or weakened tests and workflow edits are weeder's rules, read from the diff.
- **Not a writer of workflows, git hooks or the garden's lock file.** Those are the stem's. coldframe reads and reports.
- **Not the verifier.** It reports an exit code and what leaked, and never writes "pass" (tend2 does).
- **Not a pacer of agents or of the subscription.** That is the gardener's and well's.
- **Not a server, daemon or scheduler.** It is one command, run by job hooks, by LaunchAgents the owner installs, and by hand.

## Guardrails

Each one gets a test that breaks it once. `docs/engineering.md`, "Rules and their guards", names the test, or the slice that lands it.

1. It changes no project's tests, workflows, hooks or lock.
2. It deletes only what a runner made inside `~/actions-runner-*`, or what a `run` scope created, and it says which. It uses `trash` by default. Deleting for good waits for the owner's stated exception.
3. It never restarts a busy runner.
4. A slot can slow CI, never stall it: a wait has a ceiling, and a dead holder's slot is reclaimed.
5. Job hooks never call GitHub and never read a key.
6. It refuses a self-hosted runner on a public repository.
7. It spends nothing.
8. Its own data has a lifetime.

## Shape

The language is Rust, as one binary: hooks run in the runner's session with nothing guaranteed on PATH, once per job. The core is pure functions from data to data, and the edges read the disk, the process table and `gh`. Tests drive the core with real values, and the edges with real temp folders and real processes. They never mock what they test. `tests/shape.rs` holds the line between core and edge (`docs/architecture.md`).

## Tests

The suite runs in under a second, so every change runs all of it. A changed-only selection waits until the suite passes 30 seconds.

| Moment | Command | What it runs | Measured |
|---|---|---|---|
| Every pull request | `scripts/test.sh`, then `scripts/hygiene.sh` | the whole suite, then a run killed part-way, each failing if anything is left behind | 0.2 to 0.5 s warm, about 3 s from a cold build as CI builds; 0.3 to 0.6 s for the killed run |
| A pull request touching what a heavy test measures | none yet | heavy tests arrive with slices 1b, 3a and 5d, each with its own command | |
| The main line after each merge | the same as every pull request, on the push to master | everything | |
| A release | none yet: the owner tags, then `cargo install --locked --git` at the tag on a clean machine | | |

Measured 2026-11-05 on the owner's Mac (18 cores, load average 9 to 18 from other work): 20 runs of the suite and 5 of the killed run, then the same with 18 busy loops added. None failed. On GitHub's hosted macOS runner, CI's job runs in 23 to 36 s, but the wait for a free runner is what varies: none to 10 minutes on pull request 1, 2026-11-05. CI also runs `cargo fmt --check`, `cargo clippy --locked --all-targets -- -D warnings` and `shellcheck scripts/*.sh` (`docs/engineering.md`, "Where CI runs"). The Rust version is `rust-toolchain.toml`, which CI and every checkout read.

## Hard limits

- Delete with `trash`, never `rm`. A run may remove a temp folder it created itself.
- Never `git reset`. Never force-push. Never touch the git stash.
- Never echo a secret.
- No em dashes in prose.
- No mocks, stubs, TODOs, fallbacks or shims without the owner's word. Never delete or skip a failing test.
- Name no competing product in the repository's prose.
- Work only in this repository and its private notes repository. Changes in another bed go to that bed's owner, through plotplot (the umbrella agent) on mycelium.

## How work lands

Every change is a branch and a pull request, a failing test first, every CI job green, then a merge. This repository is public, so its CI runs on GitHub's hosted macOS runners, which cost nothing for a public repository. It never gets a runner on the owner's Mac: a stranger's pull request would run code there (guardrail 6). `tests/workflow.rs` holds this.

Nothing private goes in this repository: no path on the owner's machine, and no name of the owner's private projects (`tests/public.rs`). The garden's own tools may be named, as the public umbrella names them. Evidence about the owner's machine and repositories (the research copies, measure reports, spike notes) goes to the private notes repository, jahala/coldframe-notes.
