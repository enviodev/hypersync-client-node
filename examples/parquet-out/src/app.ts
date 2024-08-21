import { DataType, HexOutput, HypersyncClient, LogField } from "@envio-dev/hypersync-client";

async function main() {
  // Create hypersync client using the mainnet hypersync endpoint
  const client = HypersyncClient.new({
    url: "https://eth.hypersync.xyz",
  });

  // The query to run
  const query = {
    "fromBlock": 18500123,
    "toBlock": 18501123,
    "logs": [{
      "address": ["0xdAC17F958D2ee523a2206206994597C13D831ec7"],
      "topics": [["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"]],
    }],
    // Select the fields we are interested in, notice topics are selected as topic0,1,2,3
    "fieldSelection": {
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
    },
  };

  console.log("Downloading data into parquet... This might take some time depending on connection speed");

  await client.collectParquet("data", query, {
    /// Convert binary columns to prefixed hex format like '0x1ab..'
    hexOutput: HexOutput.Prefixed,
    columnMapping: {
      decodedLog: {
        "value": DataType.Float64,
      },
    },
    eventSignature: "Transfer(address indexed from, address indexed to, uint256 value)",
  });

  console.log("finished writing parquet");
}

main();
