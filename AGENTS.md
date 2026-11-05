# coldframe

coldframe is the bench where a change's tests run before it goes out. It owns how tests run on the owner's own machines: the self-hosted runners and the load they put on the machine, each project's declared test tiers and their measured times, and the hygiene of each run (a run leaves nothing behind, and holds on a loaded machine). It is one tool in the plotplot garden (umbrella `jahala/plotplot`), in its Infrastructure row.

Read first, in this order:
1. This page.
2. `HANDOVER.md`: the brief, the first milestone and the build order.
3. The design: `jahala/plotplot` `docs/ideas/coldframe.md`.
4. The garden's law: `jahala/plotplot` `CLAUDE.md` and `docs/building-the-garden.md`.

## What coldframe is not

- **Not a judge of test changes.** Deleted, skipped or weakened tests and workflow edits are weeder's rules, read from the diff.
- **Not a writer of workflows, git hooks or `garden.lock`.** Those are the stem's. coldframe reads and reports.
- **Not the verifier.** It reports an exit code and what leaked, and never writes "pass" (tend2 does).
- **Not a pacer of agents or of the subscription.** That is the gardener's and well's.
- **Not a server, daemon or scheduler.** It is one command, run by job hooks, by LaunchAgents the owner installs, and by hand.

## Guardrails (each one has a test that breaks it once)

1. It changes no project's tests, workflows, hooks or lock.
2. It deletes only what a runner made inside `~/actions-runner-*`, or what a `run` scope created, and it says which. It uses `trash` by default. Deleting for good waits for the owner's stated exception.
3. It never restarts a busy runner.
4. A slot can slow CI, never stall it: a wait has a ceiling, and a dead holder's slot is reclaimed.
5. Job hooks never call GitHub and never read a key.
6. It refuses a self-hosted runner on a public repository.
7. It spends nothing.
8. Its own data has a lifetime.

## Shape

The language is Rust, as one binary: hooks run in the runner's session with nothing guaranteed on PATH, once per job. The core is pure functions from data to data, and the edges read the disk, the process table and `gh`. Tests drive the core with real values, and the edges with real temp folders and real processes. They never mock what they test.

## Hard limits

- Delete with `trash`, never `rm`. A run may remove a temp folder it created itself.
- Never `git reset`. Never force-push. Never touch the git stash.
- Never echo a secret.
- No em dashes in prose.
- No mocks, stubs, TODOs, fallbacks or shims without the owner's word. Never delete or skip a failing test.
- Name no competing product in the repository's prose.
- Work only in this repository. Changes in another bed go to that bed's owner, through plotplot (the umbrella agent) on mycelium.

## How work lands

Every change is a branch and a pull request, a failing test first, every CI job green, then a merge. This repository is private, so its hosted jobs draw on the owner's monthly GitHub minutes, and GitHub refuses them once those run out. Its CI runs on self-hosted runners on the owner's Mac labelled `coldframe`, with `runs-on: ${{ fromJSON(vars.CI_RUNS_ON || '["self-hosted", "macOS", "coldframe"]') }}` and the TMPDIR step, exactly as graft's and well's CI do. Ask plotplot to set the runners up when your first workflow is ready. The owner decides if and when it goes public.
