# Handover: build coldframe

For the agent who builds coldframe. Written 2026-10-30 by plotplot, the umbrella agent of the plotplot garden. Read this whole page, then `AGENTS.md`, then the design (`jahala/plotplot` `docs/ideas/coldframe.md`). Contact plotplot on mycelium (id `plotplot`) with your plan before building. Arm two watchers first: a Monitor on `node mycelium.mjs --watch <your id>`, and a tail of `/tmp/mycelium/<your id>/journal.jsonl`. Take the id `coldframe` with `mycelium_rename`.

## Who it is for, and their pain

An owner who builds many projects with agents, and runs their tests and CI on their own Mac to keep paid minutes near zero. On 2026-10-29:
- 29 runners on an 18-core Mac held 35 GB, 6.5 GB of it logs and old runner versions nothing removes.
- One project's heavy jobs queued about 10 minutes and waited up to 19 more for a quiet machine, to run 3 to 6 minutes of tests.
- Paid minutes stopped CI twice.
- "A run leaves nothing behind" existed in six separate designs, and most of the owner's test rules had nothing checking them.

## The evidence to start from

Read these. They hold the numbers this page summarises:
- `jahala/coldframe-notes:docs/research/ci-review-2026-10-29.md`: the CI review.
- `jahala/coldframe-notes:docs/research/ci-machine-tooling-plan.md`: the first plan. Its machine design (slots as `mkdir` folders, no daemon, hooks in the runner's session) stands.
- `jahala/coldframe-notes:docs/research/coldframe-groundwork-ci-2026-10-30.md`: what exists and what guards nothing, per rule.
- `jahala/coldframe-notes:docs/research/coldframe-groundwork-garden-2026-10-30.md`: how a bed is planted, garden.json's fields, and the borders with weeder, hedge, the stem and tend2.
- `~/.claude/skills/ci-runner/` and `~/.claude/skills/test-setup/`: the mechanics coldframe replaces and the judgement it keeps as skill text.

## The first milestone

The first milestone is a tool that runs on real data, with its proof. It is steps 0 to 2 of the design's build order. Each step lands on master before the next starts.

0. **Measure, and spike the job hook.** These are scripts, before the crate grows:
   - `measure` over the last 14 days of jobs in the self-hosted repositories, minute by minute. Exclude load the owner's agents ran by hand: a 10-run stress test with 12 busy loops ran on 2026-10-29.
   - On one runner of a quiet repository: show that a started hook can block a job, that its wait counts toward `timeout-minutes`, and which variables the hook sees.
   - Report both to plotplot. The kill criterion for slots is in the design.
1. **Lifetimes and health:** `prune`, `status`, `runner adopt`. This is the disk fix.
   - Bar: tested on a copy of a real runner folder, then run on all runners, with the measured sizes dropping by the expected amount.
   - Control: a deliberately broken prune turns `status` red.
   - `prune` uses `trash` until the owner states the exception for runner-made files.
2. **The run scope:** `coldframe run`, proven on a known case.
   - Plant a leak (a stray file, a live child process, a leftover container) in copies of graft and mull, whose own leftover checks already catch leaks.
   - Bar: coldframe catches every planted leak and passes every clean run, matching the projects' own checks.

## Rules for every test (the owner's hard rule)

Before any test spends anything, or touches a real runner:
1. **List its assumptions.** For example: the hook really runs before the job; a runner folder's layout matches what you read; tests read the temp folder and ports coldframe offers.
2. **Check each at $0 on real data,** and write the result into the test's record. A failed check redesigns the test, and it does not run.
3. **Dry-run on 3 items,** reading every raw output.
4. **Then fix the bar and its control,** never to change after.

Every bar includes a control: a comparison the result must beat, never a count alone.

## What plotplot needs from you

- **One line on mycelium as each step lands, or a real question.** A question you can answer from the code, the docs or a search is not a real question.
- **A tend2 map in `docs/tend2/`:**
  - the project page carries Who, Pain, prose and a current goal;
  - one loop page per capability family A to E, each capability with its state, first test, bar and control.

  Lint it with tend2 built from `jahala/tend` at commit `c17ed38` in a temporary folder. Never use the global `tend2`.
- **`garden.json`** grows from `kind ["repository"]` to a bed's manifest when the first command ships.

## Open questions only the owner can answer

These are recorded here so you do not wait on them:
1. Deleting runner files for good, as a stated exception or through Finder's 30-day Trash. Until then, use `trash`.
2. Whether `.coldframe/times.json` is committed or git-ignored. Decide this when step 4 comes, not now.
