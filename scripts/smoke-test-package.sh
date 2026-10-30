#!/usr/bin/env bash
# Packs the library the way it is published (the main package, with the
# optionalDependencies napi adds at publish time, plus the platform package
# holding the native binary), installs the main package into a throwaway
# project and checks it can be required, imported and type-checked.
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
platform_tgz="$(cd "$work/platform" && npm pack --silent --pack-destination "$work")"
platform_name="$(node -p "require('$work/platform/package.json').name")"

# Main package: `napi pre-publish` (prepublishOnly, which npm pack doesn't run)
# adds an optionalDependency on every platform package. Reproduce that with
# napi's own target parsing so users' resolution path is what gets tested.
main_tgz="$(cd "$root" && npm pack --silent --ignore-scripts --pack-destination "$work")"
mkdir "$work/main"
tar -xzf "$work/$main_tgz" -C "$work/main"
rm "$work/$main_tgz"
(cd "$root" && node --input-type=module - "$work/main/package/package.json") <<'JS'
import { readFileSync, writeFileSync } from "node:fs";
import { parseTriple } from "@napi-rs/cli";
const path = process.argv[2];
const pkg = JSON.parse(readFileSync(path, "utf8"));
pkg.optionalDependencies = Object.fromEntries(
  pkg.napi.targets.map((t) => [`${pkg.name}-${parseTriple(t).platformArchABI}`, pkg.version]),
);
writeFileSync(path, JSON.stringify(pkg, null, 2));
JS
main_tgz="$(cd "$work/main/package" && npm pack --silent --ignore-scripts --pack-destination "$work")"

mkdir "$work/app"
cd "$work/app"
# Install only the main package; npm must resolve this platform's binary
# through its optionalDependencies (pointed at the local tarball, since this
# version may not be published yet; other platforms are skipped by os/cpu).
cat > package.json <<JSON
{
  "name": "smoke",
  "private": true,
  "dependencies": { "@envio-dev/hypersync-client": "file:$work/$main_tgz", "typescript": "5" },
  "overrides": { "$platform_name": "file:$work/$platform_tgz" }
}
JSON
npm install --silent --no-audit --no-fund >/dev/null
[ -f "node_modules/$platform_name/hypersync-client.$platform.node" ] || {
  echo "platform package $platform_name was not installed via optionalDependencies" >&2
  exit 1
}

version="$(node -p "require('$root/package.json').version")"
[ "$(node -p "require('@envio-dev/hypersync-client/package.json').version")" = "$version" ]
# The binary must load from the platform package, not a stray local .node.
if ls node_modules/@envio-dev/hypersync-client/*.node >/dev/null 2>&1; then
  echo "main package contains a .node binary" >&2
  exit 1
fi

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
