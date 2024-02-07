import { HypersyncClient } from "@envio-dev/hypersync-client";
import fs from "node:fs";

async function main() {
    // Create hypersync client using the mainnet hypersync endpoint
    const client = HypersyncClient.new({
      url: "https://eth.hypersync.xyz"
    });

    // The query to run
    const query = {
        "fromBlock": 18500123,
        "toBlock": 18550123,
        "transactions": [{}],
        // Select the fields we are interested in, notice topics are selected as topic0,1,2,3
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
            "transaction_hash",
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

    console.log("Writing to parquet... This might take some time depending on connection speed");

    await client.createParquetFolder(query, {
      path: "data",
      /// retry internal requests forever so we don't fail on a bad connection
      retry: true,
      /// Convert binary columns to prefixed hex format like '0x1ab..'
      hexOutput: true,
    });

    console.log("finished writing parquet");
}

main();
