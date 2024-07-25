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

test("Fetches event from base", async (t) => {
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
