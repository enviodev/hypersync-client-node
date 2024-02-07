# @envio-dev/hypersync-client-node
import { HypersyncClient, Decoder } from "@envio-dev/hypersync-client";
import fs from "node:fs";

async function main() {
    // Create hypersync client using the mainnet hypersync endpoint
    const client = HypersyncClient.new({
        url: "https://eth.hypersync.xyz"
    });

    // The address we want to get all ERC20 transfers and transactions for
    const addr = "1e037f97d730Cc881e77F01E409D828b0bb14de0";

    // The query to run
    const query = {
        // start from block 0 and go to the end of the chain (we don't specify a toBlock).
        "fromBlock": 0,
        // The logs we want. We will also automatically get transactions and blocks relating to these logs (the query implicitly joins them).
        "logs": [
          {
            "topics": [
              // We want ERC20 transfers
              ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
              [],
              // We want the transfers that go to this address.
              // appending zeroes because topic is 32 bytes but address is 20 bytes
              [
                "0x000000000000000000000000" + addr
              ]
            ]
          },
          {
            "topics": [
              // We want ERC20 transfers
              ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
              // We want the transfers that come from this address
              // appending zeroes because topic is 32 bytes but address is 20 bytes
              [
                "0x000000000000000000000000" + addr
              ],
              []
            ]
          }
        ],
        "transactions": [
          // We want all the transactions that come from this address
          {
            "from": [
              "0x" + addr
            ],
          },
          // We want all the transactions that went to this address
          {
            "to": [
              "0x" + addr
            ],
          }
        ],
        // Select the fields we are interested in
        "fieldSelection": {
          "block": [
            "number",
            "timestamp",
            "hash"
          ],
          "log": [
            "block_number",
            "log_index",
            "transaction_index",
            "data",
            "address",
            "topic0",
            "topic1",
            "topic2",
            "topic3"
          ],
          "transaction": [
            "block_number",
            "transaction_index",
            "hash",
            "from",
            "to",
            "value",
            "input"
          ]
        },
      };

    // run the query once
    const res = await client.sendReq(query);
    console.log(JSON.stringify(res, null, 2));

    // read json abi file for erc20
    const abi = fs.readFileSync('./erc20.abi.json', 'utf8');
    const parsedAbi = JSON.parse(abi);

    //console.log(JSON.stringify(parsedAbi, null, 2));

    let abis = {};

    // every log we get should be decodable by this abi but we don't know
    //  the specific contract addresses since we are indexing all erc20 transfers.
    for (const log of res.data.logs) {
      abis[log.address] = parsedAbi;
    }

    // Create a decoder based on our abi file
    const decoder = Decoder.new(abis);

    // Decode the logs using the decoder
    const decodedLogs = decoder.decodeLogs(res.data.logs);

    console.log(JSON.stringify(decodedLogs, (_, v) => typeof v === 'bigint' ? v.toString() : v, 2));

    // Create a parquet folder by running this query and writing the contents to disk
    await client.createParquetFolder(query, "data");
    console.log("finished writing parquet folder");
}

main();