import test from "ava";
import { Decoder, HypersyncClient } from "../index.js";

test("Decodes event from etherscan", async (t) => {
  const decoder = Decoder.fromSignatures([
    "event Mint(address sender, address indexed owner, int24 indexed tickLower, int24 indexed tickUpper, uint128 amount, uint256 amount0, uint256 amount1)",
  ]);

  const log = {
    topics: [
      "0x7a53080ba414158be7ec69b987b5fb7d07dee101fe85488f0853ae16239d0bde",
      "0x000000000000000000000000827922686190790b37229fd06084350e74485b72",
      "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
      "0x0000000000000000000000000000000000000000000000000000000000000001",
    ],
    data: "0x000000000000000000000000827922686190790b37229fd06084350e74485b72000000000000000000000000000000000000000000000000000000000bebae76000000000000000000000000000000000000000000000000000000000000270f000000000000000000000000000000000000000000000000000000000000270f",
  };
  const decoded = await decoder.decodeLogs([log]);
  t.is(decoded[0].indexed[1].val, -1n);
});

const TRANSFER_SIG = "event Transfer(address indexed from, address indexed to, uint256 value)";
const FROM = "0xdac17f958d2ee523a2206206994597c13d831ec7";
const TO = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
const transferLog = {
  address: "0x1234567890123456789012345678901234567890",
  topics: [
    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
    `0x${FROM.slice(2).padStart(64, "0")}`,
    `0x${TO.slice(2).padStart(64, "0")}`,
  ],
  data: `0x${(1000000).toString(16).padStart(64, "0")}`,
};

test("Decodes a Transfer log", async (t) => {
  const decoder = Decoder.fromSignatures([TRANSFER_SIG]);
  const [decoded] = decoder.decodeLogsSync([transferLog]);
  t.deepEqual(
    decoded.indexed.map((v) => v.val),
    [FROM, TO],
  );
  t.is(decoded.body[0].val, 1000000n);
  t.deepEqual(await decoder.decodeLogs([transferLog]), [decoded]);
  t.deepEqual(decoder.decodeEventsSync([{ log: transferLog }]), [decoded]);
  t.deepEqual(await decoder.decodeEvents([{ log: transferLog }]), [decoded]);
});

// #10: decoding is keyed on the event signature, so the log address (or its
// absence) must not matter.
test("Decoding does not depend on the log address", (t) => {
  const decoder = Decoder.fromSignatures([TRANSFER_SIG]);
  const expected = decoder.decodeLogsSync([transferLog]);
  for (const address of ["0x0000000000000000000000000000000000000000", undefined]) {
    t.deepEqual(decoder.decodeLogsSync([{ ...transferLog, address }]), expected);
  }
});

// #18: checksummed address output.
test("Decoder checksums addresses when enabled", (t) => {
  const checksummed = ["0xdAC17F958D2ee523a2206206994597C13D831ec7", "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"];
  const vals = (decoder) => decoder.decodeLogsSync([transferLog])[0].indexed.map((v) => v.val);

  t.deepEqual(vals(Decoder.fromSignaturesWithChecksum([TRANSFER_SIG], true)), checksummed);
  const decoder = Decoder.fromSignatures([TRANSFER_SIG]);
  decoder.enableChecksummedAddresses();
  t.deepEqual(vals(decoder), checksummed);
  decoder.disableChecksummedAddresses();
  t.deepEqual(vals(decoder), [FROM, TO]);
});

test("Undecodable logs yield null instead of throwing", (t) => {
  const decoder = Decoder.fromSignatures([TRANSFER_SIG]);
  const logs = [
    { ...transferLog, topics: [`0x${"11".repeat(32)}`, ...transferLog.topics.slice(1)] },
    { ...transferLog, topics: [null] },
    { ...transferLog, data: undefined },
    { ...transferLog, data: "0xzz" },
  ];
  t.deepEqual(decoder.decodeLogsSync(logs), [null, null, null, null]);
});

test("Invalid signatures are rejected", (t) => {
  t.throws(() => Decoder.fromSignatures(["event Foo("]));
});

//skig the async test against hypersync endpoint
test.skip("Fetches event from base", async (t) => {
  const decoder = Decoder.fromSignatures([
    "event Mint(address sender, address indexed owner, int24 indexed tickLower, int24 indexed tickUpper, uint128 amount, uint256 amount0, uint256 amount1)",
  ]);
  const client = HypersyncClient.new({ url: "https://base.hypersync.xyz" });
  const res = await client.getEvents({
    fromBlock: 13899663,
    toBlock: 13899664,
    logs: [
      {
        address: ["0x98c7A2338336d2d354663246F64676009c7bDa97"],
        topics: [
          [
            "0x7a53080ba414158be7ec69b987b5fb7d07dee101fe85488f0853ae16239d0bde",
          ],
        ],
      },
    ],
    fieldSelection: {
      log: [
        "topic0",
        "topic1",
        "topic2",
        "topic3",
        "data",
        "log_index",
        "transaction_hash",
      ],
    },
  });

  const decoded = await decoder.decodeEvents(res.data);
  t.is(decoded[0].indexed[1].val, -1n);
  // console.log(decoded[0].indexed[1].val);
  // const decoder = Decoder.fromSignatures([
  //   "event Mint(address sender, address indexed owner, int24 indexed tickLower, int24 indexed tickUpper, uint128 amount, uint256 amount0, uint256 amount1)",
  // ]);
  //
  // const log = {
  //   topics: [
  //     "0x7a53080ba414158be7ec69b987b5fb7d07dee101fe85488f0853ae16239d0bde",
  //     "0x000000000000000000000000827922686190790b37229fd06084350e74485b72",
  //     "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
  //     "0x0000000000000000000000000000000000000000000000000000000000000001",
  //   ],
  //   data: "0x000000000000000000000000827922686190790b37229fd06084350e74485b72000000000000000000000000000000000000000000000000000000000bebae76000000000000000000000000000000000000000000000000000000000000270f000000000000000000000000000000000000000000000000000000000000270f",
  // };
  // const decoded = await decoder.decodeLogs([log]);
  // t.is(decoded[0].indexed[1].val, -1n);
});
