#!/usr/bin/env bash
# Type-checks every example against the local build, and with --run also runs
# each one against HyperSync (needs ENVIO_API_TOKEN).
#
# Each example gets RUN_SECONDS. Examples that run indefinitely (listed below)
# pass if they are still running cleanly at that point; the rest must exit 0
# within it. Any ERROR logged by the client (e.g. a failed request being
# retried) fails the example either way, since the client retries forever.
#
# Usage: scripts/check-examples.sh [--run] [example dir...]
set -uo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
run=false
if [ "${1:-}" = "--run" ]; then run=true; shift; fi
run_seconds="${RUN_SECONDS:-60}"
long_running=" all-erc20 call_watch height-stream reverse_wallet watch "

if $run && [ -z "${ENVIO_API_TOKEN:-}" ]; then
  echo "ENVIO_API_TOKEN must be set to run the examples" >&2
  exit 1
fi

if [ $# -eq 0 ]; then set -- "$root"/examples/*/; fi

failed=()
for dir in "$@"; do
  name="$(basename "$dir")"
  echo "::group::$name"
  ok=true
  (
    set -e
    cd "$dir"
    # Links @envio-dev/hypersync-client to the repo root ("../..").
    npm install --no-audit --no-fund --no-package-lock --silent
    npx tsc -p . --outDir dist
  ) || ok=false

  if $ok && $run; then
    log="$(mktemp)"
    (cd "$dir" && timeout --preserve-status -s TERM "$run_seconds" node dist/app.js) 2>&1 | tee "$log"
    status=${PIPESTATUS[0]}
    # 143 = stopped by our SIGTERM after RUN_SECONDS.
    if [ $status -eq 143 ] && [[ "$long_running" == *" $name "* ]]; then
      status=0
    fi
    if [ $status -ne 0 ]; then
      echo "::error::example $name exited with status $status"
      ok=false
    elif grep -q " ERROR " "$log"; then
      echo "::error::example $name logged errors"
      ok=false
    fi
    rm -f "$log"
  fi
  rm -rf "$dir/dist" "$dir/node_modules"
  echo "::endgroup::"
  $ok || failed+=("$name")
done

if [ ${#failed[@]} -gt 0 ]; then
  echo "failed examples: ${failed[*]}" >&2
  exit 1
fi
echo "all examples ok"
