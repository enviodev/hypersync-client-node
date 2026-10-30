// #62: setLogLevel. The logger is process-global and initialised by the
// client constructor, so each case runs in a fresh child process.
import test from "ava";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const indexJs = fileURLToPath(new URL("../index.js", import.meta.url));

// Makes one request that fails (connection refused), which the client logs at ERROR.
const script = `
const { HypersyncClient, setLogLevel } = require(${JSON.stringify(indexJs)});
for (const level of process.argv.slice(1)) setLogLevel(level);
new HypersyncClient({ url: "http://127.0.0.1:1", apiToken: "x", maxNumRetries: 0 })
  .getHeight()
  .then(() => process.exit(2), () => process.exit(0));
`;

function errorLogLines(levels, env = {}) {
  const { RUST_LOG, ...baseEnv } = process.env;
  const res = spawnSync(process.execPath, ["-e", script, ...levels], { env: { ...baseEnv, ...env }, encoding: "utf8" });
  if (res.status !== 0) throw new Error(`child exited ${res.status}: ${res.stderr}`);
  return res.stderr.split("\n").filter((l) => l.includes("ERROR")).length;
}

test("logs errors by default", (t) => {
  t.true(errorLogLines([]) > 0);
});

test("setLogLevel('off') silences logging", (t) => {
  t.is(errorLogLines(["off"]), 0);
});

test("setLogLevel('error') keeps errors", (t) => {
  t.true(errorLogLines(["error"]) > 0);
});

test("RUST_LOG takes precedence over setLogLevel", (t) => {
  t.is(errorLogLines(["error"], { RUST_LOG: "off" }), 0);
});

test("setLogLevel tolerates repeat calls and junk input", (t) => {
  t.notThrows(() => errorLogLines(["warn", "error", "!!!=???"]));
});
