import test from "ava";
import { readFileSync } from "node:fs";
import { HypersyncClient } from "../index.js";
import { startFakeServer, clientConfig, RATE_LIMIT_HEADERS } from "./_fake-server.mjs";

const pkg = JSON.parse(readFileSync(new URL("../package.json", import.meta.url)));

async function setup(t, serverOpts, clientOpts) {
  const server = await startFakeServer(serverOpts);
  t.teardown(server.close);
  return { server, client: new HypersyncClient(clientConfig(server.url, clientOpts)) };
}

test("height and chain id requests", async (t) => {
  const { client } = await setup(t);
  t.is(await client.getHeight(), 42);
  t.is(await client.getChainId(), 7);
});

// #44 / #65: the user agent must identify this binding with the real package
// version (it was hscn/0.0.0 because napi pins the Cargo version).
test("sends hscn/<package version> user agent and bearer token", async (t) => {
  const { server, client } = await setup(t);
  await client.getHeight();
  const { headers } = server.requests[0];
  t.is(headers["user-agent"], `hscn/${pkg.version}`);
  t.not(pkg.version, "0.0.0");
  t.is(headers.authorization, "Bearer tok-123");
});

test("newWithAgent overrides the user agent", async (t) => {
  const server = await startFakeServer();
  t.teardown(server.close);
  const client = HypersyncClient.newWithAgent(clientConfig(server.url), "custom/1");
  await client.getHeight();
  t.is(server.requests[0].headers["user-agent"], "custom/1");
});

test("constructor validates config", (t) => {
  t.throws(() => new HypersyncClient({ url: "", apiToken: "x" }), { message: /url/ });
  t.throws(() => new HypersyncClient({ url: "http://127.0.0.1:1" }), { message: /apiToken/ });
});

// #57: rate limit headers are surfaced and, by default, respected.
test("rate limit info is parsed from response headers", async (t) => {
  const { client } = await setup(t);
  t.is(client.rateLimitInfo(), null);
  const res = await client.getWithRateLimit({ fromBlock: 1, fieldSelection: {} });
  const expected = { limit: 50, remaining: 0, resetSecs: 41, cost: 10 };
  t.deepEqual(res.rateLimit, expected);
  t.is(res.response ?? null, null);
  t.deepEqual(client.rateLimitInfo(), expected);
});

test("proactive rate limit sleep avoids hitting the server", async (t) => {
  const { server, client } = await setup(
    t,
    { rateLimitHeaders: { ...RATE_LIMIT_HEADERS, "x-ratelimit-reset": "1" } },
    { proactiveRateLimitSleep: true },
  );
  const query = { fromBlock: 1, fieldSelection: {} };
  await client.getWithRateLimit(query);
  await client.getWithRateLimit(query);
  t.is(server.queries().length, 1, "second call should be short-circuited by the known rate limit");

  const start = Date.now();
  await client.waitForRateLimit();
  t.true(Date.now() - start >= 500);
  await client.getWithRateLimit(query);
  t.is(server.queries().length, 2);
});

test("without proactive sleep every call reaches the server", async (t) => {
  const { server, client } = await setup(t);
  const query = { fromBlock: 1, fieldSelection: {} };
  await client.getWithRateLimit(query);
  await client.getWithRateLimit(query);
  t.is(server.queries().length, 2);
});
