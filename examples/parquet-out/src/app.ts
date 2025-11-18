import { HypersyncClient, Query } from "@envio-dev/hypersync-client";

async function main() {
  // Create hypersync client using the mainnet hypersync endpoint
  const client = new HypersyncClient({
    url: "https://eth.hypersync.xyz",
    apiToken: process.env.ENVIO_API_TOKEN!,
  });

  // The query to run
  const query: Query = {
    fromBlock: 18500123,
    toBlock: 18501123,
    logs: [
      {
        address: ["0xdAC17F958D2ee523a2206206994597C13D831ec7"],
        topics: [
          [
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
          ],
        ],
      },
    ],
    // Select the fields we are interested in, notice topics are selected as topic0,1,2,3
    fieldSelection: {
      log: [
        "BlockNumber",
        "LogIndex",
        "TransactionIndex",
        "TransactionHash",
        "Data",
        "Address",
        "Topic0",
        "Topic1",
        "Topic2",
        "Topic3",
      ],
    },
  };

  console.log(
    "Downloading data into parquet... This might take some time depending on connection speed",
  );

  await client.collectParquet("data", query, {
    /// Convert binary columns to prefixed hex format like '0x1ab..'
    hexOutput: "Prefixed",
    columnMapping: {
      decodedLog: {
        value: "Float64",
      },
    },
    eventSignature:
      "Transfer(address indexed from, address indexed to, uint256 value)",
  });

  console.log("finished writing parquet");
}

main();
