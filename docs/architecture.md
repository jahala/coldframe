# Architecture

coldframe is one Rust binary. It runs from a runner's job hooks, from LaunchAgents the owner installs, and by hand. It is never a server, a daemon or a scheduler. `AGENTS.md` says what it is for and what it is not. This page says how its parts fit and where its data lives.

## Shape

The core decides, and the edge does the I/O.

- **The core** is every file under `src/` except the edge. Its functions take data and return data: which slot to take, what to prune, what a run left behind, which survey findings hold. It does not touch the disk, the clock, the environment, the process table, the network or the terminal.
- **The edge** is `src/main.rs`, and the files under src/edge/ once the first command that reads the machine lands (slice 1a). It reads what the core needs, passes it in, and acts on the answer: it prints, moves files to the Trash, starts and stops processes, and calls `gh`.
- **Time, the home folder and every root folder are passed in.** The core never asks for today's date or for `~`. Tests pass their own.
- **One door to GitHub.** Every `gh` call will go through one edge function with a list of the endpoints it may call. Job hooks never reach it.

`tests/shape.rs` holds this rule: it fails when a core file reaches for I/O, and it proves its detector on a sample that must be flagged.

## Built so far

Only the command's conventions: `coldframe --version` and `coldframe --help`. `src/lib.rs` reads the arguments and `src/main.rs` prints the answer. The commands arrive in the slices of `docs/build-plan.md`, and each one joins the usage line in `src/lib.rs`.

## Exit codes

| Code | Meaning | From |
|---|---|---|
| 0 | done, or healthy | now |
| 1 | unhealthy, or the run it watched failed or left something behind | slice 1a |
| 2 | a usage error; nothing is written | now |
| 3 | coldframe could not read what it needs, so it cannot say healthy or not | slice 1a |

A failure to read is never reported as healthy or unhealthy. `tests/cli.rs` holds codes 0 and 2.

## What coldframe may write and delete

- **Write:** its own folders (below), and a project's own .coldframe folder. Nothing else in a project, ever (guardrail 1).
- **Delete:** only what a runner made inside `~/actions-runner-*` (old logs in `~/actions-runner-*/_diag`, superseded `bin.<version>` and `externals.<version>` folders, and `~/actions-runner-*/_work/_update`), and what a `run` scope created. It moves them to the Trash, and says which. Deleting for good waits for the owner's stated exception.
- `tests/hard_limits.rs` holds that coldframe's own code calls no deletion today. When prune lands, its one Trash function becomes the only allowed call, and the test names it.

## Data and its lifetime

Every store that grows says what it keeps and what removes the rest. The rule lands with the store, in the same slice, with a test.

| Store | Where | Keeps | Removed by | Lands in |
|---|---|---|---|---|
| Prune record | `~/Library/Application Support/coldframe/prune.json` | the latest prune's outcome only | each prune overwrites it | 1c |
| Logs | `~/Library/Logs/coldframe/` | 30 days | `prune` | 1c |
| Slots | `~/Library/Application Support/coldframe/slots/` | at most the slot count | `hook completed`; a dead holder's slot is reclaimed | 5a |
| Survey reports | `~/Library/Application Support/coldframe/survey/` | the last 12 | `survey`, as it writes the 13th | 6 |
| Measured times | a project's .coldframe/times.json | the latest per tier | `time` overwrites it | 4b |
| Config | `~/.config/coldframe/config.toml` | the owner's settings | the owner | 1c |
| Test runs | `$TMPDIR/coldframe-tests/run-<pid>/` | one run's temporary files | the run itself; a killed run's by the next run | now |

## Dependencies

None yet. Each one added names why in its pull request, and the standard library comes first.
