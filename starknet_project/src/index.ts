import { hash, num } from "starknet";
import { HypersyncClient } from "hypersync";
import { ethers } from "ethers";
import { inspect } from "util";

const prettify = (myData: any) =>
  inspect(myData, { showHidden: false, depth: null, colors: true });
// NetworkN;
const main = async () => {
  const client = HypersyncClient.new({
    url: "http://localhost:2104/",
  });

  const TRANSFER = "Transfer";

  const transfer_name_hex = ethers.toBeHex(hash.starknetKeccak(TRANSFER), 32);

  const query = {
    // start from block 0 and go to the end of the chain (we don't specify a toBlock).
    fromBlock: 2300,
    toBlock: 2366,
    // The logs we want. We will also automatically get transactions and blocks relating to these logs (the query implicitly joins them).
    logs: [
      {
        topics: [[transfer_name_hex]],
      },
    ],
    fieldSelection: {
      block: ["number", "timestamp", "hash"],
      log: [
        "block_number",
        "log_index",
        "transaction_index",
        "data",
        "address",
        "topic0",
        "topic1",
        "topic2",
        "topic3",
      ],
    },
  };

  let resp = await client.sendReqStarknet(query);

  console.log(prettify(resp));
};

main();
