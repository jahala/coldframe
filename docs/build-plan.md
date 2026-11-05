# Build plan: coldframe in slices

2026-11-05. Written by coldframe's agent after reading `HANDOVER.md`, `AGENTS.md`, the design (`jahala/plotplot` `docs/ideas/coldframe.md` at `58f1e54`), the garden's law, the four research files (now in `jahala/coldframe-notes:docs/research/`), the ci-runner and test-setup skills, and the machine itself. It follows the design's build order (steps 0 to 7) and cuts each step into slices. A slice is one pull request: a failing test first, every CI job green, then a merge. One pull request is open at a time. A step that depends on another waits for it to land; step 1 does not depend on step 0.

## What the machine says today

These numbers were read on 2026-11-05. Some differ from the research files, and the plan uses these.

| Fact | Research said | Today |
|---|---|---|
| Runner folders | 29 | 31, for 17 repositories (one has 4) |
| Runner disk | 35 GB | 39 GB |
| Free disk | 66 GB, 93% full | 191 GB, 79% full |
| Prunable now | about 6.5 GB | about 8.5 GB: `_diag` older than 7 days 3.9 GB, superseded versions 2.5 GB on 5 runners, `_work/_update` 2.1 GB on the same 5 |
| Jobs running | | 0 `Runner.Worker`, yet load average 10.5 on 18 cores |
| Hooks | `after-job.sh` on 28 of 29 | completed hook only, on 30 of 31; one runner has none; no started hook anywhere |

Facts that change how the code must be written:
- `.runner` starts with a UTF-8 byte-order mark. A strict JSON parse fails on it.
- `bin` and `externals` are links only on the 5 runners that updated themselves. On the rest they are real folders, and a real folder is always the current version.
- Every runner also holds `.runner_migrated`, a copy of `.runner` in the newer format.
- Folder names are lowercased, while plist labels keep GitHub's case, and one runner's folder has no number.
- A runner reads `.env` only when it starts, so adopting a hook means restarting the runner, and that is allowed only while it is idle.

Facts that weaken the design's bars (plotplot ruled on these, `docs/decisions.md` 2):
- **Step 2's known case covers files only.** mull's check compares the work tree and its own temp folder before and after. graft has no end-of-run check at all: each test removes its own scratch folder. Neither project checks processes or containers.
- **No project reads a port range.** Both use port 0. So the scope's port range has no consumer to prove it on, and step 2 proves the temp folder only.
- **The machine is loaded with no CI job running.** This supports the design's warning that the load may be the agents', which step 0's kill criterion tests.

## The order, in one view

| Step | Slices | Needs first |
|---|---|---|
| F. Foundations | F1 the crate and the law's checks · F2 the tend2 map | nothing |
| 0. Measure and spike | 0a `measure` script and report · 0b started-hook spike | F1; 0b needs a runner on the private notes repository |
| 1. Lifetimes and health | 1a `status`, first cut · 1b `prune --dry-run` · 1c `prune` · 1d `runner adopt` and `hook completed` | F1; it does not wait on step 0 |
| 2. Run scope | 2a files · 2b processes · 2c containers · 2d the proof on graft and mull | step 1 |
| 3. Under load | 3a `run --load` and its proof | step 2 |
| 4. Test plan | 4a the `tests` contract (plotplot's) · 4b `time` · 4c `check plan` · 4d `affected` | step 3; 4a lands in the umbrella |
| 5. Weighted slots | 5a slot core · 5b `hook started` · 5c `run` takes a slot · 5d burst proof | step 0's kill criterion passed |
| 6. Survey | 6a workflow traps · 6b runner safety · 6c cost | step 5, or step 4 if slots are dropped |
| 7. Faces and hand-over | 7a skill · 7b `runner add` and `remove` · 7c pre-push guard · 7d projects move over | step 6 |

## Rules every slice keeps

The testing doctrine in `docs/engineering.md` binds every slice: the failing test first, real values and never mocks, the pre-flight rule, and a guard broken once. Two more rules for the plan:
- **A bar always has a control.** A count alone never passes.
- **One line on mycelium to plotplot as each step lands.**

## F. Foundations

**F1. The foundations.** The crate, the documents, and a guard for each rule they state, so coldframe is a bed the fit contract can check before its first command. What landed:
- the crate, pinned to Rust 1.99.1, whose binary only names its version and refuses a usage error with exit 2;
- `docs/architecture.md`, `docs/engineering.md` (with the rules-and-guards table) and `docs/decisions.md`;
- tests for the core page and cited paths, the hard limits, the shape, the workflow rules and the exit codes, each detector proved on a sample;
- `scripts/test.sh`, which fails a run that leaves anything behind, and `scripts/hygiene.sh`, which proves a killed run leaves nothing after the next one;
- `.github/workflows/ci.yml`, on the owner's runners with the way back to hosted ones.

CI runs on GitHub's hosted macOS runners, because the repository is public (`docs/decisions.md` 7).

**F2. The tend2 map.** `docs/tend2/coldframe.tend2.html` carries Who, Pain, prose and the current goal. Five family pages, one each for A to E, carry the capability table (id, capability, state, first test, bar, control, page). CI lints it with tend2 built from `jahala/tend` at `c17ed38` in a temporary folder.
- First test: a check that every capability carries a state, a first test, a bar and a control, and that no id repeats. graft's `scripts/checks/capabilities.sh` is the model.
- It needs a `TEND_DEPLOY_KEY` secret in coldframe's repository, which plotplot or the owner sets.

## 0. Measure and spike

**0a. `measure`, as a script.** `scripts/measure.sh` (bash, `gh`, `jq`). For each of the 17 self-hosted repositories, it lists the last 14 days of runs and their jobs. From each job's created, started and completed times it builds a minute-by-minute timeline: jobs queued and jobs running, per repository and for the whole machine.
- Assumptions to check at $0: the jobs API returns `runner_name` and `labels` for every job, so self-hosted jobs can be told apart; 14 days of runs stay within the API's rate limit; the 2026-10-29 stress test can be found and excluded by its runs.
- The instrument is checked on a known case first. Run over the review's window, it must reproduce the review's numbers for the heaviest project's heavy jobs (queued about 10 minutes, then waiting up to 19 for a quiet machine).
- Output: `jahala/coldframe-notes:docs/research/measure-<date>.md`, private because it names the owner's repositories, with the kill-criterion answer. Slots are dropped if CI ran above the cap (6 jobs, a third of the cores) in under 5 percent of busy minutes. A busy minute is one with at least one job running.
- The script's raw answers for 3 repositories over 3 days are kept as a fixture. When `measure` moves into the binary, its Rust test must give the script's answer on that fixture. Then the script goes.

**0b. The started-hook spike.** coldframe is public, so it gets no runner on the owner's Mac. The spike runs instead on a runner of the private notes repository, jahala/coldframe-notes, so no other project or its workflows are touched. A spike pull request there (a draft, closed, never merged) adds a job with `timeout-minutes: 1`. For the spike only, the runner's `.env` gets a started hook that records what it sees and then waits.
- Questions it answers:
  - Does a started hook block the job?
  - Does its wait count toward `timeout-minutes`?
  - Which variables does it see? (`GITHUB_JOB`, job-level `env`, and any sign of the job's labels)
  - Must the hook be a `.sh` file, or can it be the binary?
  - Can it find its `Runner.Worker` process by walking up its parents?
  - Does a `caffeinate -w <worker pid>` it starts survive the runner's orphan cleanup?
- The hook records variable names, and values only for a short list that holds no secret. The `.env` is restored after, and the runner is restarted only while idle.
- Output: `jahala/coldframe-notes:docs/research/spike-started-hook-<date>.md`. It must record whether the hook can see the job's labels, because that decides how a job's weight reaches the hook: a label if it can, else the job id.

Both results go to plotplot on mycelium.

## 1. Lifetimes and health

**1a. `status`, first cut.** It lists every runner folder:
- its repository and name, read from `.runner` (byte-order mark allowed);
- online (a `Runner.Listener` for that folder is alive), busy (a `Runner.Worker` under it) or idle;
- its disk use;
- which hooks its `.env` names;
- its version, from the link or from the real folder.

It exits 0 when healthy, 1 when not, and 2 on a usage error. `garden.json` becomes a bed's manifest in this slice, with the umbrella's schema copied and pinned, and a test that it validates.
- First test: a real copy of a runner folder, with no `Runner.Listener`, reads as offline and turns `status` red.

**1b. `prune --dry-run`.** A pure function takes a folder's listing, its links, whether the runner is busy and today's date. It returns what would go, why, and how many bytes. The rules:
- `_diag` logs whose name date is older than 7 days. A file without a runner's name pattern stays, and is reported.
- `bin.<version>` and `externals.<version>` that the `bin` and `externals` links no longer point to. When `bin` is a real folder, nothing is superseded.
- `_work/_update`, only when the runner is idle and its link points at the newest version.
- coldframe's own logs older than 30 days.
- Bar: run on all 31 runners, the dry-run predicts about 8.5 GB. The control is the independent `du` and `find` count made on 2026-11-05.

**1c. `prune`.** It applies the plan with `trash`, one idle runner at a time, checking for a `Runner.Worker` just before each runner. It records each run's outcome under `~/Library/Application Support/coldframe/`, keeping only the latest. `status` turns red when the last prune failed or is older than 2 days. The repository ships a LaunchAgent plist for a daily prune, which the owner installs, and `status --notify` posts a macOS notification only when unhealthy.
- Bar: tested first on a copy of a real runner folder, then run on all runners. Each folder's measured size drops by its dry-run prediction, within 1 percent. The control is a copied folder coldframe was not run on, whose size must not move.
- Control for health: a prune broken on purpose (it cannot reach one runner folder) turns `status` red.
- While the owner has not stated the exception, the space frees only when the Trash is emptied, because Finder's 30-day setting is off.

**1d. `runner adopt` and `hook completed`.** `hook completed` does what `after-job.sh` does today: `git clean -ffdxq` of the checkout, only inside the runner's `_work`. This is the one stated exception, kept as it is. `adopt` points each runner's completed hook at coldframe and restarts it only while it is idle. The runner with no hook gets one.
- `hook completed` is the stand-in for `after-job.sh`, so it is checked against the real thing. Both run on identical copies of a dirty checkout, and must leave the same tree.
- If 0b shows a hook must be a `.sh` file, the hook is a two-line script that runs the binary. That is the runner's interface, and it needs the owner's word before it ships.

## 2. Run scope

`coldframe run -- <argv>` runs a command inside a scope coldframe opens. Named tiers come in step 4.

**2a. Files.** The scope gives the command its own `TMPDIR` and records the work tree and the system temp folder before and after. After the run, it reports any file outside the scope, moves its own temp folder to the Trash, and fails the run if anything leaked.

**2b. Processes.** Every process the command starts carries a scope marker in its environment, and the scope reads the process table for it. A process still alive after the run is reported and stopped. The scope started it, so stopping it is allowed.

**2c. Containers.** It lists containers and images before and after, and reports and removes the new ones.

**2d. The proof on known cases.** Files are proved against mull's own check. graft is replaced: the first task is to find which of the owner's three private projects with leftover checks has an end-of-run check that catches leftover processes or containers, and to use it for those kinds.
- Bar: in copies of the chosen projects, coldframe catches every planted leak and passes every clean run, with the same verdicts as each project's own check.
- If no project checks processes or containers, those two kinds are proved against planted leaks alone: a clean run must pass, and one run leaks each kind once. That result is recorded as weaker evidence, because no outside check backs it.
- Pre-flight: count how many of each project's tests take their temp folder from the environment. graft follows `TMPDIR`. mull uses `MULL_TMP`, which its test script makes itself.
- Kill criterion: if the scope cannot reproduce mull's verdicts on files, the per-project checks stay and C1 is dropped.

## 3. Under load

**3a. `run --load`.** It runs the same command while busy loops hold every core.
- Bar: the umbrella's `two_beds_run_at_the_same_time`, at the commit before plotplot PR 189 (timed) and after it (rendezvous). Under `--load`, the old version fails and the new one passes, 10 runs of 10 each.
- Control: without `--load`, the old version passes. This proves it is the load that fails it.

## 4. The test plan

**4a. The `tests` contract.** A block in `garden.json` beside `smoke`: each tier has a name, an argv, when it runs (each change, heavy, main line, release), a weight, whether it needs a clean machine, and the paths it covers. It is a change in the umbrella, so coldframe drafts it and plotplot lands it. coldframe then copies the schema and pins it.

**4b. `time`.** It runs each tier in a run scope and records its time and the machine under `.coldframe/`. The owner decides then whether `.coldframe/times.json` is committed.

**4c. `check plan`.** It reports when the test strategy in a project's CLAUDE.md does not name every declared tier, or states a time outside twice the measured one.
- Bar: it flags a stale time planted in a copy of a CLAUDE.md and passes the true one.

**4d. `affected`.** Given the changed files and the tiers' paths, it returns the tiers a change must run. When unsure, it returns all of them.

The two projects that declare their tiers do so through their own owners, by way of plotplot.

## 5. Weighted slots (only if 0a passes)

**5a. The slot core.** Slot folders live under `~/Library/Application Support/coldframe/slots/`. Each is taken with `mkdir`, which is atomic, and holds its weight, runner, run, job and holder process. A pure function chooses a slot from the folders and the live processes. A dead holder's slot is reclaimed. A wait has a ceiling, then the job runs and an overflow is counted.

**5b. `hook started`.** It reads the job's weight by the route 0b chose, takes a slot, keeps the Mac awake for the job, and gives the slot back on `hook completed`. `adopt` adds it to each runner.

**5c. `run` takes a slot too,** so agents' local runs and CI share one cap.

**5d. The proof.** Small batch first, on two repositories.
- Bar: under a burst from three repositories, the weight in use never exceeds the cap, and a killed job's slot is reclaimed within one poll.
- Control: the same burst with slots off exceeds the cap.
- Then `measure` again, against 0a's numbers.

## 6. Survey

`survey`, renamed from the design's `audit` because pleach and tend both have an `audit`.

It reports, and never changes anything. It runs from a LaunchAgent in the owner's session, where `gh` can read the Keychain. It keeps the last 12 reports.
- **6a. Workflow traps.** The ci-runner skill's five traps, read from every repository's workflow files.
- **6b. Runner safety.** Self-hosted runners on public repositories, private ones with outside collaborators, and runners on repositories idle for 30 days.
- **6c. Cost.** The month's paid minutes from the billing API, by repository and job.
- Bar: tested on workflow files and API answers recorded today, and once against the live API. It finds the review's open items and nothing the review cleared.

## 7. Faces and hand-over

- **7a. The skill.** How an agent runs a project's tests with `coldframe run`, and what a red result means. It replaces the mechanics of the ci-runner and test-setup skills, and keeps their judgement as text.
- **7b. `runner add` and `runner remove`.** These replace the skill's scripts. `add` refuses a public repository.
- **7c. The pre-push guard,** through the stem's git dispatch. It is optional.
- **7d. Projects move over one at a time,** each with `measure` before and after. The heaviest project's quiet-machine script and its three runner variables go if its heavy tiers pass under the cap. Idle repositories lose their runners.
- Kill criterion for size: once projects have moved over, coldframe must be smaller in total than what it replaced.

## Guardrails

Which test holds each guardrail, and which slice lands it, is in `docs/engineering.md`, "Rules and their guards". plotplot's rulings on this plan are decisions 1 to 4 in `docs/decisions.md`.

## Questions for the owner

1. **Deleting runner files for good.** Prune uses `trash` until you decide. With Finder's 30-day Trash setting off, the space frees only when you empty the Trash. The recommendation stands: a stated exception, limited by code to runner-made files inside `~/actions-runner-*`. The disk is less urgent than the review found (191 GB free), so this can wait until step 1c. The same answer covers the test runner: today, each run of `scripts/hygiene.sh` on your Mac moves one killed run's folder to the Trash (`docs/decisions.md` 6). The recommendation is that the exception also covers coldframe's own test-run folders.
2. **The `.coldframe/times.json` question** comes at step 4, not now.
