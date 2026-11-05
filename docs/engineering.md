# Engineering

How coldframe is written and tested. The test commands and their measured times are in `AGENTS.md`, "Tests". The shape and the data lifetimes are in `docs/architecture.md`.

## Testing doctrine

- **The failing test comes first.** A slice starts as a test that fails for the reason the slice exists.
- **Real values, never mocks.** The core is driven with real values recorded from this machine. The edge is driven with real temporary folders and real processes. A copy of a runner folder never carries `.credentials` or `.credentials_rsaparams`.
- **A check that reads text proves itself.** Each detector in `tests/` is shown a sample it must flag and one it must pass.
- **A guard is proved by breaking it once.** Remove or weaken the guarded thing, watch its test fail, restore it, and say so in the pull request.
- **Tests hold on a loaded machine.** They wait for the event, never a fixed time. A deadline only catches a hang. Time, ids, the home folder and every root folder are passed in. No test claims a fixed port, name or path.
- **The pre-flight rule.** Before a test spends anything or touches a real runner, its assumptions are listed and checked at $0 on real data, it is dry-run on 3 items with every raw output read, and only then are its bar and control fixed. The record goes in docs/preflight/, one file per slice, created with the first slice that needs one.
- **A flaky test gets a root cause.** No retry, no skip, no looser assertion. Loop it until it fails and explain why.
- **Heavy tests get their own command.** A test that needs a copy of a real runner folder, a loaded machine or the machine to itself is a separate test target with its own script, kept out of `scripts/test.sh`. It is never marked ignored: `tests/hard_limits.rs` refuses a skipped test.
- **Output is clean.** A passing run prints only cargo's lines and the scripts' last line.

## A run leaves nothing behind

- `scripts/test.sh` points `TMPDIR` at a folder of its own, `$TMPDIR/coldframe-tests/run-<pid>`, and runs `cargo test`. It fails if anything is left in that folder, or if the working tree's status changed, ignored files included. cargo's build output in `target/` is kept between runs on purpose.
- Each test makes its temporary folder through `tests/common/mod.rs`, which removes it when the test ends. Removing a temporary folder the run made itself is the one removal a run may make.
- A run killed outright cannot clean up. The next run moves the folder of any earlier run whose process is gone to the Trash.
- `scripts/hygiene.sh` proves it: it kills a run part-way, checks the killed run's folder is still there, runs the suite again, and checks that nothing is left.

## Rules and their guards

This table is the one home for which check holds each rule. "Nothing yet" names the slice that lands the guard (`docs/build-plan.md`). When a guard lands, its row changes in the same pull request.

| Rule (where it is stated) | Held by | State |
|---|---|---|
| Guardrail 1: changes no project's tests, workflows, hooks or lock (`AGENTS.md`) | every command run against a project copy, whose tree must not change | nothing yet: 2a, and again in 6a |
| Guardrail 2: deletes only runner-made or scope-made files, with `trash` | `tests/hard_limits.rs` (no deletion call in `src/`); later, a user file planted in a runner's `_diag` and beside the runner folder both survive prune | partly: no deletion code exists; the prune test lands in 1b and 1c |
| Guardrail 3: never restarts a busy runner | a real process named `Runner.Worker` under a copied folder blocks `adopt` | nothing yet: 1d |
| Guardrail 4: a slot can slow CI, never stall it | a slot held by a dead process is reclaimed; a full cap lets the job run after the ceiling | nothing yet: 5a |
| Guardrail 5: hooks never call GitHub or read a key | hooks run with a `gh` on PATH that records any call, and an empty home folder | nothing yet: 1d and 5b |
| Guardrail 6: refuses a self-hosted runner on a public repository | `runner add` refuses on a recorded public repository's answer | nothing yet: 7b |
| Guardrail 7: spends nothing | the one door to GitHub refuses any endpoint not on its list | nothing yet: 1a and 6c |
| Guardrail 8: its own data has a lifetime | a test per store in `docs/architecture.md`, "Data and its lifetime" | partly: test runs, by `scripts/test.sh` and `scripts/hygiene.sh`; the rest land with each store |
| Never writes "pass" (`AGENTS.md`, "What coldframe is not") | no command's output carries a pass verdict | nothing yet: 2a |
| Delete with `trash`, never `rm` (hard limits) | `tests/hard_limits.rs` for coldframe's code; the owner's hedge policy for agents | held |
| Never `git reset`, never force-push, never touch the stash | the owner's hedge policy, outside this repository | held outside |
| Never echo a secret | the owner's hedge policy, outside this repository | held outside |
| No em dashes in prose | `tests/hard_limits.rs` | held |
| No mocks, stubs, placeholders or skipped tests | `tests/hard_limits.rs`: placeholder and skip markers in code, mock dependencies in `Cargo.toml` | partly: a fallback or a shim cannot be read from text; review holds it |
| Never delete a failing test | weeder, at commit, outside this repository | held outside |
| Name no competing product | nothing | nothing yet |
| Work only in this repository | nothing | nothing yet |
| The core does no I/O (`docs/architecture.md`, "Shape") | `tests/shape.rs` | held |
| The core page within 150 lines, a one-line `CLAUDE.md`, and every cited path resolves (the garden's fit check F7) | `tests/references.rs` | held |
| Every CI job runs on GitHub's hosted macOS runners and never on a self-hosted one, the TMPDIR step first, and actions pinned to a commit (`AGENTS.md`, "How work lands") | `tests/workflow.rs` | held |
| No path on the owner's machine and no name of a private project in any file (`AGENTS.md`, "How work lands") | `tests/public.rs`, which keeps the names only as hashes | held for files; commit messages were checked once, on 2026-11-05, and nothing holds them yet |
| Usage errors exit 2 and write nothing (`docs/architecture.md`, "Exit codes") | `tests/cli.rs` | held |
| The README's examples run and print what they show (`README.md`) | `tests/readme.rs` | held |
| A run leaves nothing behind, even when killed | `scripts/test.sh` and `scripts/hygiene.sh` | held |
| Every CI job is green before a merge (`AGENTS.md`, "How work lands") | a branch rule on master: the `check` job must pass on the change merged with the latest master, for the owner too, and master takes no force-push or deletion | held, since 2026-11-05 |

## Where CI runs

`.github/workflows/ci.yml` runs one job on GitHub's hosted macOS runners. The repository is public, so those cost nothing, and it never runs on a runner on the owner's Mac, where a stranger's pull request would run code as the owner. macOS, because coldframe reads macOS runner folders and LaunchAgents. On a pull request the checkout is the change merged into master, so the checks run on what the merge would make.

The job runs, in order, stopping at the first failure:
1. `cargo fmt --check`
2. `cargo clippy --locked --all-targets -- -D warnings`
3. `shellcheck scripts/*.sh`, with shellcheck 0.11.0 installed first and checked against its SHA-256, because GitHub's macOS image has none
4. `scripts/test.sh`
5. `scripts/hygiene.sh`

## Recipes

### Run every check, as CI does

```sh
cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && shellcheck scripts/*.sh && scripts/test.sh && scripts/hygiene.sh
```

### Add a rule

State it once, in the doc that owns its topic. Add its guard in the same pull request, with a sample the guard must flag. Add its row to "Rules and their guards". If nothing can hold it yet, the row says so and names the slice that will.
