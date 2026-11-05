#!/usr/bin/env bash
# Runs the suite, then fails if the run left anything behind (docs/engineering.md, "A run
# leaves nothing behind"). Every test makes its temporary files under TMPDIR, which this run
# points at its own folder, coldframe-tests/run-<this script's pid>. That folder must be empty
# when the suite ends, and the working tree's status, ignored files included, must be what it
# was before. target/ is cargo's build output, kept between runs on purpose.
#
# A run killed outright cannot clean up after itself, so each run first moves to the Trash the
# folder of any earlier run whose process is gone. scripts/hygiene.sh proves this.
set -euo pipefail
cd "$(dirname "$0")/.."

runs="${TMPDIR:-/tmp}"
runs="${runs%/}/coldframe-tests"
mkdir -p "$runs"
for old in "$runs"/run-*; do
  [ -d "$old" ] || continue
  if ! kill -0 "${old##*/run-}" 2>/dev/null; then
    trash "$old"
    echo "moved a dead run's folder to the Trash: $old" >&2
  fi
done

tree_status() { git status --porcelain --ignored | grep -v -x -F '!! target/' || true; }

before="$(tree_status)"
scratch="$runs/run-$$"
mkdir "$scratch"
suite=0
TMPDIR="$scratch" cargo test --locked --no-fail-fast "$@" || suite=$?

left_in_scratch="$(ls -A "$scratch")"
left_in_tree="$(comm -13 <(printf '%s\n' "$before" | sort) <(tree_status | sort))"
if [ -z "$left_in_scratch" ]; then
  rmdir "$scratch"
  rmdir "$runs" 2>/dev/null || true
fi

if [ -n "$left_in_scratch" ] || [ -n "$left_in_tree" ]; then
  echo "The run left files behind:" >&2
  if [ -n "$left_in_scratch" ]; then printf '  %s: %s\n' "$scratch" "$left_in_scratch" >&2; fi
  if [ -n "$left_in_tree" ]; then printf '  %s\n' "$left_in_tree" >&2; fi
  exit 1
fi
exit "$suite"
