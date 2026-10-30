#!/usr/bin/env bash
# Type-checks every example against the local build, and with --run also runs
# them all in parallel against HyperSync (needs ENVIO_API_TOKEN) to check data
# flows through end to end.
#
# When running:
# - examples that finish must exit 0 within RUN_TIMEOUT seconds;
# - examples that stream forever (listed in `ready_pattern` below) pass as soon
#   as they print their first real output, and are then stopped;
# - anything still running at RUN_TIMEOUT is treated as hung;
# - any error logged fails the example, since the client retries failed
#   requests forever and some examples catch and print their own errors.
#
# Usage: scripts/check-examples.sh [--run] [example dir...]
set -uo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
run=false
if [ "${1:-}" = "--run" ]; then run=true; shift; fi
run_timeout="${RUN_TIMEOUT:-60}"

# Output that proves a streaming example received data.
declare -A ready_pattern=(
  [call_watch]="Transaction decoded|no tx"
  [height-stream]="Height: [0-9]+"
  [reverse_wallet]="hash: '0x"
  [watch]="scanned up to [0-9]+"
)
error_pattern=' ERROR |Error:|Error in '

if $run && [ -z "${ENVIO_API_TOKEN:-}" ]; then
  echo "ENVIO_API_TOKEN must be set to run the examples" >&2
  exit 1
fi

if [ $# -eq 0 ]; then set -- "$root"/examples/*/; fi

logs="$(mktemp -d)"
trap 'rm -rf "$logs"' EXIT

# Prints "pass" or a failure reason as its last line.
check() {
  local dir="$1" name
  name="$(basename "$dir")"
  cd "$dir" || return
  trap 'rm -rf dist node_modules' RETURN

  # Links @envio-dev/hypersync-client to the repo root ("../..").
  npm install --no-audit --no-fund --no-package-lock --silent || { echo "npm install failed"; return; }
  npx tsc -p . --outDir dist || { echo "type check failed"; return; }
  $run || { echo pass; return; }

  local out="$logs/$name.out" pattern="${ready_pattern[$name]:-}" start=$SECONDS
  node dist/app.js >"$out" 2>&1 &
  local pid=$!
  local result=""
  while [ -z "$result" ]; do
    sleep 0.5
    if grep -Eq "$error_pattern" "$out"; then
      result="logged an error"
    elif ! kill -0 "$pid" 2>/dev/null; then
      wait "$pid" && result=pass || result="exited with status $?"
      if [ "$result" = pass ] && [ -n "$pattern" ] && ! grep -Eq "$pattern" "$out"; then
        result="exited without printing /$pattern/"
      fi
    elif [ -n "$pattern" ] && grep -Eq "$pattern" "$out"; then
      result=pass
    elif [ $((SECONDS - start)) -ge "$run_timeout" ]; then
      result="still running after ${run_timeout}s (hung?)"
    fi
  done
  kill "$pid" 2>/dev/null
  wait "$pid" 2>/dev/null
  head -c 4000 "$out"
  echo "$result ($((SECONDS - start))s)"
}

for dir in "$@"; do
  check "$dir" >"$logs/$(basename "$dir").log" 2>&1 &
done
wait

failed=()
for dir in "$@"; do
  name="$(basename "$dir")"
  result="$(tail -n 1 "$logs/$name.log")"
  echo "::group::$name: $result"
  cat "$logs/$name.log"
  echo "::endgroup::"
  if [[ "$result" != pass* ]]; then
    echo "::error::example $name: $result"
    failed+=("$name")
  fi
done

if [ ${#failed[@]} -gt 0 ]; then
  echo "failed examples: ${failed[*]}" >&2
  exit 1
fi
echo "all examples ok"
