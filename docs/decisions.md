# Decisions

Each decision with its context, what was chosen and why. Numbered in order, never renumbered. A decision that is replaced stays, with a line naming the one that replaced it.

## 1. Build in slices, one pull request at a time

Replaced in part by decision 8: the plan's home is now the tend2 map, and its slices are the map's capabilities. One pull request at a time still holds.

2026-11-05. The design's build order has eight steps, some too big for one review. Each step is cut into slices in a build plan, and each slice is one pull request with its failing test first. One pull request is open at a time. A step waits only for the steps it depends on, so step 1 (prune and status) does not wait for step 0 (measure and spike). plotplot ruled this on 2026-11-05: the design's "step 1 does not wait on step 0" wins over the handover's "each step lands before the next".

## 2. Step 2's known cases: mull for files, and graft replaced

2026-11-05. The design named graft and mull as the known cases for the run scope. On 2026-11-05 graft had no end-of-run leftover check, and neither project checked processes or containers. plotplot ruled: prove files against mull's own check, and replace graft with whichever of the owner's three private projects with leftover checks has an end-of-run check for processes or containers. If none has one, those two kinds are proved against planted leaks with a control (a clean run passes, one run leaks each kind once), and the result is recorded as weaker evidence. plotplot fixes the design's line about graft.

## 3. The started-hook spike runs on coldframe's own runner

Replaced by decision 7: a public repository gets no runner on the owner's Mac.

2026-11-05. The design put the spike on a runner of a quiet private repository. Running it on coldframe's own runner changes no other repository or workflow. plotplot agreed, on the condition that the spike records whether the hook can see the job's labels, because that decides how slots read a job's weight.

## 4. `audit` becomes `survey`, and `runner adopt` stays

2026-11-05. pleach and tend both have an `audit`, and the garden reads the word as theirs. plotplot chose `survey`, what a gardener does to a bed, and makes the same change in the design. `runner adopt` stays, because "runner" is GitHub's own word for what it adopts.

## 5. The core does no I/O, and a test holds it

2026-11-05. The garden's shape is pure functions in the core and I/O at the edge. Written as prose only, it drifts. `tests/shape.rs` fails when a file under `src/`, other than `src/main.rs` and the edge's files, reaches for the disk, the clock, the environment, the process table, the network or the terminal. It reads text, so it can miss a use it has no marker for. It proves its detector on a sample.

## 6. A killed test run is cleaned by the next run, through the Trash

2026-11-05. A run killed outright cannot remove its own temporary folder. The next run of `scripts/test.sh` moves the folder of any earlier run whose process is gone to the Trash. It uses `trash`, not `rm`, because the folder is not one that run made itself, and the hard limit allows `rm` only for that. `scripts/hygiene.sh` kills a run part-way to prove it, in CI as well. If `trash` fails in a runner's session, that check fails loudly, and this decision is revisited. The cost: each run of the check on the owner's Mac adds one small folder to the owner's Trash (CI's hosted runners are thrown away after each job), and Finder's 30-day setting is off, so nothing removes it. The owner's answer to the open question on the map's bench page, whether prune may delete for good, settles this as well.

## 7. coldframe is public, from a clean history, and its CI runs on GitHub

2026-11-05. The owner chose to publish coldframe before building, so its CI runs on GitHub's hosted macOS runners at no cost, and none of it on the owner's Mac. That reverses the choice of 2026-11-03 to keep it private.

- **A new repository, not a rewritten one.** The old history carried paths on the owner's machine and details of private projects. GitHub keeps every pull request's commits under refs nobody can delete, so rewriting master would not remove them, and force-pushing is a hard limit besides. The old repository is renamed jahala/coldframe-notes and stays private, with its whole history and pull requests. A new jahala/coldframe starts from the same commits, rewritten with the research copies removed and private names and paths replaced.
- **Nothing private in the public repository.** `tests/public.rs` fails on a path on the owner's machine or the name of a private project. Evidence about the owner's machine goes to the notes repository.
- **CI never runs on the owner's Mac.** `tests/workflow.rs` refuses a self-hosted runner, so guardrail 6 holds for coldframe itself.
- **What it costs.** The started-hook spike (step 0b) needs a self-hosted runner, and cannot use coldframe's own. It moves to a runner on the private notes repository. plotplot confirmed this on 2026-11-05, and registers that runner when step 0b is ready.

## 8. The plan lives on the tend2 map, and the page shows the finished tool

2026-11-05. The owner, through plotplot-design, asked for coldframe's work to be planned in tend2, like the garden's other tools, and for the project page to describe the finished tool under an "under construction" ribbon.

- **The map replaces the build plan.** `docs/tend2/` holds a project loop, a persona, an opportunity and one loop per piece of the first milestone: measure, the started-hook spike, status, prune, adopt and the run scope. Each loop's bar and control are in its checks' claims. Work not decided yet waits in the project loop's `## Tried`, since the map holds only decided bets. The build plan and its slice names are gone.
- **The installed tend2 lints the map.** The handover pinned tend at `c17ed38` and forbade the installed `tend2`. On 2026-11-05 the installed one was tend's newest commit, `484ffdf`, six weeks ahead of that pin, and the owner asked why the old one was being built. Local lints use the installed `tend2`, and CI pins `484ffdf` so both read the map alike.
- **The page shows the finished tool, honestly.** The hero's frame previews what `coldframe status` will print, titled "preview", beside the ribbon. `tests/page.rs` fails the day a real command ships while the ribbon or the preview is still there.
