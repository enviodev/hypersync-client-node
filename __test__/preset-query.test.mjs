import test from "ava";
import {
  presetQueryLogs,
  presetQueryLogsOfEvent,
  presetQueryBlocksAndTransactionHashes,
  presetQueryBlocksAndTransactions,
  HypersyncClient,
} from "../index.js";
import { startFakeServer, clientConfig } from "./_fake-server.mjs";

const USDT = "0xdac17f958d2ee523a2206206994597c13d831ec7";
const TRANSFER = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
const ALL_LOG_FIELDS = [
  "Address", "BlockHash", "BlockNumber", "Data", "LogIndex", "Removed",
  "Topic0", "Topic1", "Topic2", "Topic3", "TransactionHash", "TransactionIndex",
];

test("presetQueryLogs", (t) => {
  const q = presetQueryLogs(USDT, 100, 200);
  t.like(q, { fromBlock: 100, toBlock: 200, logs: [{ address: [USDT] }], joinMode: 0 });
  t.deepEqual([...q.fieldSelection.log].sort(), ALL_LOG_FIELDS);
});

test("presetQueryBlocksAndTransactionHashes", (t) => {
  const q = presetQueryBlocksAndTransactionHashes(1);
  t.true(q.includeAllBlocks);
  t.deepEqual(q.transactions, [{}]);
  t.deepEqual([...q.fieldSelection.transaction].sort(), ["BlockHash", "BlockNumber", "Hash"]);
});

test("presetQueryBlocksAndTransactions", (t) => {
  const q = presetQueryBlocksAndTransactions(1, 2);
  t.like(q, { fromBlock: 1, toBlock: 2, includeAllBlocks: true });
});

test("preset queries validate their inputs", (t) => {
  t.throws(() => presetQueryLogs("0x12", 0), { message: /parse address/ });
  t.throws(() => presetQueryLogs(USDT, -1));
});

test("preset queries round-trip through the client", async (t) => {
  const server = await startFakeServer();
  t.teardown(server.close);
  const client = new HypersyncClient(clientConfig(server.url));
  await client.getWithRateLimit(presetQueryLogsOfEvent(USDT, TRANSFER, 100, 200));
  const [sent] = server.queries();
  t.like(sent, { from_block: 100, to_block: 200 });
  t.deepEqual(sent.logs, [{ address: [USDT], topics: [[TRANSFER]] }]);
  t.true(sent.field_selection.log.includes("log_index"));
});
