# Decisions

Each decision with its context, what was chosen and why. Numbered in order, never renumbered. A decision that is replaced stays, with a line naming the one that replaced it.

## 1. Build in slices, one pull request at a time

2026-11-05. The design's build order has eight steps, some too big for one review. Each step is cut into slices in `docs/build-plan.md`, and each slice is one pull request with its failing test first. One pull request is open at a time. A step waits only for the steps it depends on, so step 1 (prune and status) does not wait for step 0 (measure and spike). plotplot ruled this on 2026-11-05: the design's "step 1 does not wait on step 0" wins over the handover's "each step lands before the next".

## 2. Step 2's known cases: mull for files, and graft replaced

2026-11-05. The design named graft and mull as the known cases for the run scope. On 2026-11-05 graft had no end-of-run leftover check, and neither project checked processes or containers. plotplot ruled: prove files against mull's own check, and replace graft with whichever of the owner's three private projects with leftover checks has an end-of-run check for processes or containers. If none has one, those two kinds are proved against planted leaks with a control (a clean run passes, one run leaks each kind once), and the result is recorded as weaker evidence. plotplot fixes the design's line about graft.

## 3. The started-hook spike runs on coldframe's own runner

2026-11-05. The design put the spike on a runner of a quiet private repository. Running it on coldframe's own runner changes no other repository or workflow. plotplot agreed, on the condition that the spike records whether the hook can see the job's labels, because that decides how slots read a job's weight.

## 4. `audit` becomes `survey`, and `runner adopt` stays

2026-11-05. pleach and tend both have an `audit`, and the garden reads the word as theirs. plotplot chose `survey`, what a gardener does to a bed, and makes the same change in the design. `runner adopt` stays, because "runner" is GitHub's own word for what it adopts.

## 5. The core does no I/O, and a test holds it

2026-11-05. The garden's shape is pure functions in the core and I/O at the edge. Written as prose only, it drifts. `tests/shape.rs` fails when a file under `src/`, other than `src/main.rs` and the edge's files, reaches for the disk, the clock, the environment, the process table, the network or the terminal. It reads text, so it can miss a use it has no marker for. It proves its detector on a sample.

## 6. A killed test run is cleaned by the next run, through the Trash

2026-11-05. A run killed outright cannot remove its own temporary folder. The next run of `scripts/test.sh` moves the folder of any earlier run whose process is gone to the Trash. It uses `trash`, not `rm`, because the folder is not one that run made itself, and the hard limit allows `rm` only for that. `scripts/hygiene.sh` kills a run part-way to prove it, in CI as well. If `trash` fails in a runner's session, that check fails loudly, and this decision is revisited. The cost: each run of the check, in CI too, adds one small folder to the owner's Trash, and Finder's 30-day setting is off, so nothing removes it. The owner's answer to open question 1 in `docs/build-plan.md` settles this as well.
