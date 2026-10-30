import test from "ava";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { CallDecoder } from "../index.js";

const TRANSFER = "transfer(address to, uint256 amount)";
const TO = "0xdac17f958d2ee523a2206206994597c13d831ec7";
const TO_CHECKSUMMED = "0xdAC17F958D2ee523a2206206994597C13D831ec7";
const transferInput = `0xa9059cbb${TO.slice(2).padStart(64, "0")}${(1000000).toString(16).padStart(64, "0")}`;
const vals = (decoded) => decoded.map((v) => v.val);

// The upstream decoder decoded the calldata including its 4-byte selector,
// shifting every argument (call_watch printed addresses like 0x00000000c782...).
test("decodes call arguments", async (t) => {
  const decoder = CallDecoder.fromSignatures([TRANSFER]);
  t.deepEqual(vals(decoder.decodeImpl(transferInput)), [TO, 1000000n]);
  t.deepEqual(vals(decoder.decodeInputsSync([transferInput])[0]), [TO, 1000000n]);
  t.deepEqual(vals((await decoder.decodeInputs([transferInput]))[0]), [TO, 1000000n]);
  t.deepEqual(vals(decoder.decodeTransactionsInputSync([{ input: transferInput }])[0]), [TO, 1000000n]);
  t.deepEqual(vals((await decoder.decodeTracesInput([{ input: transferInput }]))[0]), [TO, 1000000n]);
});

test("checksums addresses when enabled", (t) => {
  t.deepEqual(vals(CallDecoder.fromSignaturesWithChecksum([TRANSFER], true).decodeImpl(transferInput)), [TO_CHECKSUMMED, 1000000n]);
  const decoder = CallDecoder.fromSignatures([TRANSFER]);
  decoder.enableChecksummedAddresses();
  t.is(decoder.decodeImpl(transferInput)[0].val, TO_CHECKSUMMED);
  decoder.disableChecksummedAddresses();
  t.is(decoder.decodeImpl(transferInput)[0].val, TO);
});

const undecodable = [
  "0x", // e.g. a plain ETH transfer
  "0x12",
  "zz",
  "0xa9059cbb", // selector without arguments
  `0xa9059cbb${"00".repeat(10)}`, // truncated arguments
  `0xdeadbeef${"00".repeat(64)}`, // unknown selector
];

test("returns null for input it can't decode", async (t) => {
  const decoder = CallDecoder.fromSignatures([TRANSFER]);
  const expected = undecodable.map(() => null);
  t.deepEqual(decoder.decodeInputsSync(undecodable), expected);
  t.deepEqual(await decoder.decodeInputs(undecodable), expected);
  t.deepEqual(decoder.decodeTransactionsInputSync([{}, { input: "0x" }]), [null, null]);
});

// Short input used to panic inside a tokio task, and the .unwrap() on the
// JoinError aborted the whole Node process. Run it in a child process so a
// regression fails the test instead of killing the runner.
test("short input does not crash the process", (t) => {
  const indexJs = fileURLToPath(new URL("../index.js", import.meta.url));
  const script = `
    const { CallDecoder } = require(${JSON.stringify(indexJs)});
    const d = CallDecoder.fromSignatures([${JSON.stringify(TRANSFER)}]);
    d.decodeTransactionsInput([{ input: "0x" }]).then((r) => console.log(JSON.stringify(r)));
  `;
  const res = spawnSync(process.execPath, ["-e", script], { encoding: "utf8" });
  t.is(res.status, 0, res.stderr);
  t.is(res.stdout.trim(), "[null]");
});

test("rejects invalid signatures", (t) => {
  t.throws(() => CallDecoder.fromSignatures(["transfer(address"]));
});
