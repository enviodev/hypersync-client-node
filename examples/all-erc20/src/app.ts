import { HypersyncClient, Decoder, BlockField, LogField, TransactionField } from "@envio-dev/hypersync-client";
import fs from "node:fs";

async function main() {
  // Create hypersync client using the mainnet hypersync endpoint
  // Passing null config makes it use default
  const client = HypersyncClient.new(null);

  // The query to run
  const query = {
    // Start from block 0 and go to the end of the chain (we don't specify a toBlock).
    //   you can add a "toBlock" to limit the query to a certain range.
    "fromBlock": 0,
    // The logs we want. We will also automatically get transactions and blocks relating to these logs (the query implicitly joins them).
    "logs": [
      {
        // We want All ERC20 transfers so no address filter and only a filter for the first topic
        "topics": [
          ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
        ]
      }
    ],
    // Select the fields we are interested in, notice topics are selected as topic0,1,2,3
    //   Most of the fields below are there for demonstration purposes.
    //   The only field we use in this example is the 'log.data' + 'log.address' + 'log.topic0' so you could create a faster query by removing others.
    "fieldSelection": {
      "block": [
        BlockField.Number,
        BlockField.Timestamp,
        BlockField.Hash,
      ],
      "log": [
        LogField.BlockNumber,
        LogField.LogIndex,
        LogField.TransactionIndex,
        LogField.TransactionHash,
        LogField.Data,
        LogField.Address,
        LogField.Topic0,
        LogField.Topic1,
        LogField.Topic2,
        LogField.Topic3,
      ],
      "transaction": [
        TransactionField.BlockNumber,
        TransactionField.TransactionIndex,
        TransactionField.Hash,
        TransactionField.From,
        TransactionField.To,
        TransactionField.Value,
        TransactionField.Input,
      ]
    },
  };

  console.log("Running the query...");

  // Run the query once, the query is automatically paginated so it will return when it reaches some limit (time, response size etc.)
  //  there is a nextBlock field on the response object so we can set the fromBlock of our query to this value and continue our query until
  // res.nextBlock is equal to res.archiveHeight or query.toBlock in case we specified an end block.
  const res = await client.get(query);

  console.log(`Ran the query once. Next block to query is ${res.nextBlock}`);

  // Create a decoder with our mapping
  const decoder = Decoder.fromSignatures([
    "Transfer(address indexed from, address indexed to, uint amount)"
  ]);

  // Decode the log on a background thread so we don't block the event loop.
  // Can also use decoder.decodeLogsSync rather than using this promise api if it is more convenient.
  const decodedLogs = await decoder.decodeLogs(res.data.logs);

  // Let's count total volume, it is meaningless because of currency differences but good as an example.
  let total_volume = BigInt(0);

  for (const log of decodedLogs) {
    // skip invalid logs
    if (log === null) {
      continue;
    }
    // We know it is a bigint because of the signature
    total_volume += log.body[0].val as bigint;
  }

  const totalBlocks = res.nextBlock - query.fromBlock;

  console.log(`Total volume was ${total_volume} in ${totalBlocks} blocks in ${res.data.logs.length} transfers.`);
}

main();
