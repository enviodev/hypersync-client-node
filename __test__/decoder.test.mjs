import test from "ava";
import { Decoder } from "../index.js";
import { parseAbi } from "viem";

test("Decodes event from etherscan", async (t) => {
  const abi = parseAbi([
    "event Mint(address sender, address indexed owner, int24 indexed tickLower, int24 indexed tickUpper, uint128 amount, uint256 amount0, uint256 amount1)",
  ]);
  abi[0]["anonymous"] = false;
  console.log("abi", abi);
  const decoder = Decoder.new({
    "0x98c7A2338336d2d354663246F64676009c7bDa97": abi,
  });

  const log = {
    address: "0x98c7A2338336d2d354663246F64676009c7bDa97",
    blockNumber: 0,
    logIndex: 0,
    transactionIndex: 0,
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
