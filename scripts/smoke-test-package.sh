#!/usr/bin/env bash
# Packs the library the way it is published (the main package plus the
# platform package holding the native binary), installs both tarballs into a
# throwaway project and checks it can be required, imported and type-checked.
#
# Usage: scripts/smoke-test-package.sh <napi platform dir under npm/, e.g. linux-x64-gnu>
set -euo pipefail

platform="${1:?usage: $0 <platform, e.g. linux-x64-gnu>}"
root="$(cd "$(dirname "$0")/.." && pwd)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

binary="$root/hypersync-client.$platform.node"
[ -f "$binary" ] || { echo "missing $binary, run yarn build first" >&2; exit 1; }

# Platform package: same layout `napi artifacts` produces before publishing.
cp -r "$root/npm/$platform" "$work/platform"
cp "$binary" "$work/platform/"
(cd "$work/platform" && npm pack --silent --pack-destination "$work") >/dev/null

(cd "$root" && npm pack --silent --pack-destination "$work") >/dev/null

mkdir "$work/app"
cd "$work/app"
npm init -y >/dev/null
npm install --silent --no-audit --no-fund "$work"/envio-dev-hypersync-client-*.tgz typescript@5 >/dev/null

version="$(node -p "require('$root/package.json').version")"
[ "$(node -p "require('@envio-dev/hypersync-client/package.json').version")" = "$version" ]
# The binary must load from the platform package, not a stray local .node.
! ls node_modules/@envio-dev/hypersync-client/*.node >/dev/null 2>&1

cat > smoke.cjs <<'JS'
const assert = require("node:assert");
const lib = require("@envio-dev/hypersync-client");
for (const name of ["HypersyncClient", "Decoder", "CallDecoder", "setLogLevel", "presetQueryLogs", "JoinMode", "LogField"]) {
  assert.ok(lib[name] !== undefined, `missing export ${name}`);
}
const decoder = lib.Decoder.fromSignatures(["event Transfer(address indexed from, address indexed to, uint256 value)"]);
assert.ok(decoder);
console.log("cjs ok");
JS
node smoke.cjs

cat > smoke.mjs <<'JS'
import assert from "node:assert";
import { HypersyncClient, Decoder, presetQueryLogs } from "@envio-dev/hypersync-client";
assert.equal(typeof HypersyncClient, "function");
assert.equal(typeof Decoder.fromSignatures, "function");
assert.equal(typeof presetQueryLogs, "function");
console.log("esm ok");
JS
node smoke.mjs

cat > smoke.ts <<'TS'
import { HypersyncClient, Decoder, JoinMode, type Query, type ClientConfig } from "@envio-dev/hypersync-client";
const config: ClientConfig = { url: "https://eth.hypersync.xyz", apiToken: "token" };
const query: Query = {
  fromBlock: 0,
  logs: [{ address: ["0x0000000000000000000000000000000000000000"] }],
  fieldSelection: { log: ["Address", "Data"] },
  joinMode: JoinMode.JoinNothing,
};
export const run = async (): Promise<void> => {
  const client = new HypersyncClient(config);
  const res = await client.get(query);
  const decoder = Decoder.fromSignatures([]);
  await decoder.decodeLogs(res.data.logs);
};
TS
npx tsc --noEmit --strict --module nodenext --moduleResolution nodenext --skipLibCheck false smoke.ts
echo "types ok"
