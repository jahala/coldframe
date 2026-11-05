#!/usr/bin/env bash
# Proves that a run of scripts/test.sh killed part-way leaves nothing after the next run
# (docs/engineering.md, "A run leaves nothing behind"). It starts a run in its own process
# group, waits for the run's folder to appear, kills the whole group outright, checks that the
# folder is still there (the control: the kill left something to clean), then runs the suite
# again and checks that no run's folder is left.
set -euo pipefail
cd "$(dirname "$0")/.."

runs="${TMPDIR:-/tmp}"
runs="${runs%/}/coldframe-tests"

set -m
scripts/test.sh >/dev/null 2>&1 &
victim=$!
set +m
folder="$runs/run-$victim"

# Wait for the event, never a fixed time. The deadline (120 s) only catches a hang.
for _ in $(seq 1 1200); do
  [ -d "$folder" ] && break
  sleep 0.1
done
if [ ! -d "$folder" ]; then
  kill -KILL -- "-$victim" 2>/dev/null || true
  echo "hygiene: the run never made its folder" >&2
  exit 1
fi
kill -KILL -- "-$victim"
wait "$victim" 2>/dev/null || true

if [ ! -d "$folder" ]; then
  echo "hygiene: the control failed, the killed run's folder is already gone" >&2
  exit 1
fi

scripts/test.sh

if [ -e "$runs" ] && [ -n "$(ls -A "$runs")" ]; then
  echo "hygiene: left behind in $runs: $(ls -A "$runs")" >&2
  exit 1
fi
echo "hygiene: a killed run left nothing after the next run"
