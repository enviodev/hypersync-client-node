import { HypersyncClient, Decoder, BlockField, LogField, TransactionField } from "@envio-dev/hypersync-client";
import fs from "node:fs";

// The addresses we want to get data for
const addresses = [
  "0xD1a923D70510814EaE7695A76326201cA06d080F".toLowerCase(),
  "0xc0A101c4E9Bb4463BD2F5d6833c2276C36914Fb6".toLowerCase(),
  "0xa0FBaEdC4C110f5A0c5E96c3eeAC9B5635b74CE7".toLowerCase(),
  "0x32448eb389aBe39b20d5782f04a8d71a2b2e7189".toLowerCase(),
];

// Convert address to topic for filtering. Padds the address with zeroes.
function addressToTopic(address: string): string {
  return "0x000000000000000000000000" + address.slice(2, address.length);
}

async function main() {
  // Create hypersync client using the mainnet hypersync endpoint
  const client = HypersyncClient.new({
    url: "https://eth.hypersync.xyz"
  });

  const addressTopicFilter = addresses.map(addressToTopic);

  // The query to run
  const query = {
    // start from block 0 and go to the end of the chain (we don't specify a toBlock).
    "fromBlock": 0,
    // The logs we want. We will also automatically get transactions and blocks relating to these logs (the query implicitly joins them).
    "logs": [
      {
        // We want All ERC20 transfers coming to any of our addresses
        "topics": [
          ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
          [],
          addressTopicFilter,
          [],
        ]
      },
      {
        // We want All ERC20 transfers going from any of our addresses
        "topics": [
          ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
          addressTopicFilter,
          [],
          [],
        ]
      }
    ],
    "transactions": [
      // get all transactions coming from and going to any of our addresses.
      {
        from: addresses
      },
      {
        to: addresses
      }
    ],
    // Select the fields we are interested in, notice topics are selected as topic0,1,2,3
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

  const decoder = Decoder.fromSignatures([
    "Transfer(address indexed from, address indexed to, uint amount)"
  ]);

  // Decode the log on a background thread so we don't block the event loop.
  // Can also use decoder.decodeLogsSync if it is more convenient.
  const decodedLogs = await decoder.decodeLogs(res.data.logs);

  // Let's count total volume for each address, it is meaningless because of currency differences but good as an example.
  let total_erc20_volume = {};

  for (const log of decodedLogs) {
    // skip invalid logs
    if (log === null) {
      continue;
    }

    if (!total_erc20_volume[log.indexed[0].val as string]) {
      total_erc20_volume[log.indexed[0].val as string] = BigInt(0);
    }
    if (!total_erc20_volume[log.indexed[1].val as string]) {
      total_erc20_volume[log.indexed[1].val as string] = BigInt(0);
    }
    // We count for both sides but we will filter by our addresses later so we will ignore unnecessary addresses.
    total_erc20_volume[log.indexed[0].val as string] += log.body[0].val as bigint;
    total_erc20_volume[log.indexed[1].val as string] += log.body[0].val as bigint;
  }

  for (const addr of addresses) {
    console.log(`Total erc20 transfer volume for address ${addr} is ${total_erc20_volume[addr]}`);
  }

  let total_wei_volume = {};

  for (const tx of res.data.transactions) {
    if (!total_wei_volume[tx.from]) {
      total_wei_volume[tx.from] = BigInt(0);
    }
    if (!total_wei_volume[tx.to]) {
      total_wei_volume[tx.to] = BigInt(0);
    }

    total_wei_volume[tx.from] += BigInt(tx.value);
    total_wei_volume[tx.to] += BigInt(tx.value);
  }

  for (const addr of addresses) {
    console.log(`Total wei transfer wolume for address ${addr} is ${total_wei_volume[addr]}`);
  }
}

main();
