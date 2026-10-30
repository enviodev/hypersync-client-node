// Checks the JS query -> HyperSync request conversion by capturing what the
// client sends to a fake server.
import test from "ava";
import { HypersyncClient, JoinMode, TraceField, TransactionField } from "../index.js";
import { startFakeServer, clientConfig } from "./_fake-server.mjs";

const USDT = "0xdAC17F958D2ee523a2206206994597C13D831ec7";
const TRANSFER = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
const addr = (n) => `0x${n.toString(16).padStart(40, "0")}`;

async function sentQuery(t, query) {
  const server = await startFakeServer();
  t.teardown(server.close);
  const client = new HypersyncClient(clientConfig(server.url));
  await client.getWithRateLimit(query);
  const queries = server.queries();
  t.is(queries.length, 1);
  return queries[0];
}

// #52: Either<LogFilter, LogSelection> matched {include, exclude} as an empty
// LogFilter, silently turning a selection into "match everything".
test("include/exclude selections are sent, not collapsed to empty filters", async (t) => {
  const sent = await sentQuery(t, {
    fromBlock: 1,
    logs: [
      { address: [USDT] },
      { include: { address: [USDT] }, exclude: { topics: [[TRANSFER]] } },
    ],
    transactions: [{ from: [addr(1)] }, { include: { to: [addr(2)] } }],
    traces: [{ include: { type: ["call"] }, exclude: { callType: ["delegatecall"] } }],
    blocks: [{ miner: [addr(3)] }],
    fieldSelection: {},
  });
  t.deepEqual(sent.logs, [
    { address: [USDT.toLowerCase()] },
    { address: [USDT.toLowerCase()], exclude: { topics: [[TRANSFER]] } },
  ]);
  t.deepEqual(sent.transactions, [{ from: [addr(1)] }, { to: [addr(2)] }]);
  t.deepEqual(sent.traces, [{ type: ["call"], exclude: { call_type: ["delegatecall"] } }]);
  t.deepEqual(sent.blocks, [{ miner: [addr(3)] }]);
});

// #22/#27/#43: the transaction/trace `type` field was sent as "kind", which the
// server ignores.
test("field selection uses server field names, including `type`", async (t) => {
  t.is(TransactionField.Type, "Type");
  t.is(TraceField.Type, "Type");
  t.false("Kind" in TransactionField);
  t.false("Kind" in TraceField);

  const sent = await sentQuery(t, {
    fromBlock: 1,
    fieldSelection: {
      transaction: ["Type", "Hash"],
      trace: ["Type", "From"],
      log: ["Topic0", "LogIndex"],
      block: ["Number"],
    },
  });
  t.deepEqual(sent.field_selection.transaction.sort(), ["hash", "type"]);
  t.deepEqual(sent.field_selection.trace.sort(), ["from", "type"]);
  t.deepEqual(sent.field_selection.log.sort(), ["log_index", "topic0"]);
  t.deepEqual(sent.field_selection.block, ["number"]);
});

// #30: enums must exist at runtime (no `const enum`) and map to the server's names.
test("join mode enum is usable at runtime", async (t) => {
  t.deepEqual({ ...JoinMode }, { Default: 0, JoinAll: 1, JoinNothing: 2 });
  const sent = await sentQuery(t, { fromBlock: 1, fieldSelection: {}, joinMode: JoinMode.JoinNothing });
  t.is(sent.join_mode, "JoinNothing");
});

// #37: authorization list filters.
test("authorization list filter is sent", async (t) => {
  const sent = await sentQuery(t, {
    fromBlock: 1,
    transactions: [{ authorizationList: [{ chainId: [1], address: [addr(1)] }], type: [4] }],
    fieldSelection: { transaction: ["AuthorizationList"] },
  });
  t.deepEqual(sent.transactions, [{ type: [4], authorization_list: [{ chain_id: [1], address: [addr(1)] }] }]);
  t.deepEqual(sent.field_selection.transaction, ["authorization_list"]);
});

test("invalid queries are rejected before any request is made", async (t) => {
  const server = await startFakeServer();
  t.teardown(server.close);
  const client = new HypersyncClient(clientConfig(server.url));

  const cases = [
    [{ logs: [{ address: ["0xzz"] }] }, /Failed to parse address/],
    [{ logs: [{ topics: [["0x12"]] }] }, /Failed to parse topic/],
    [{ logs: [{ topics: [[], [], [], [], []] }] }, /more than 4 topics/],
    [{ transactions: [{ sighash: ["0x1234"] }] }, /sighash/],
    [{ logs: [{ include: {}, exclude: { address: ["0xzz"] } }] }, /Failed to parse address/],
  ];
  for (const [filters, message] of cases) {
    await t.throwsAsync(client.getWithRateLimit({ fromBlock: 1, fieldSelection: {}, ...filters }), { message });
  }
  t.throws(() => client.getWithRateLimit({ fromBlock: 1, fieldSelection: { trace: ["type"] } }), {
    message: /does not match any variant of enum `TraceField`/,
  });
  t.is(server.queries().length, 0);
});
