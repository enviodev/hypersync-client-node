import { HypersyncClient, Decoder } from "@envio-dev/hypersync-client";
import fs from "node:fs";

const DAI_ADDRESS = "0x6B175474E89094C44Da98b954EedeAC495271d0F";

async function main() {
    // Create hypersync client using the mainnet hypersync endpoint
    const client = HypersyncClient.new({
      url: "https://eth.hypersync.xyz"
    });

    // The query to run
    const query = {
        // start from block 0 and go to the end of the chain (we don't specify a toBlock).
        "fromBlock": 0,
        "logs": [
          {
            "address": [DAI_ADDRESS],
            // We want the transfers
            "topics": [
              ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
              [],
              [],
              [],
            ]
          }
        ],
        // Select the fields we are interested in, notice topics are selected as topic0,1,2,3
        "fieldSelection": {
          "log": [
            "data",
            "address",
            "topic0",
            "topic1",
            "topic2",
            "topic3"
          ]
        },
      };

    // read json abi file for erc20
    const abi = fs.readFileSync('./erc20.abi.json', 'utf8');
    const parsedAbi = JSON.parse(abi);

    // Map of contract_address -> ABI
    let abis = {
      [DAI_ADDRESS]: parsedAbi,
    };

    // Create a decoder with our mapping
    const decoder = Decoder.new(abis);

    let total_dai_volume = BigInt(0);
 
    while(true) {
      const res = await client.sendReq(query);

      if(res.data.logs.length !== 0) {
        // Decode the log on a background thread so we don't block the event loop.
        // Can also use decoder.decodeLogsSync if it is more convenient.
        const decodedLogs = await decoder.decodeLogs(res.data.logs);

        for (const log of decodedLogs) {
          total_dai_volume += log.body[0].val as bigint;
        }
      }

      console.log(`scanned up to ${res.nextBlock} and total DAI transfer volume is ${total_dai_volume / BigInt(1e18)} USD`);

      if (res.archiveHeight == res.nextBlock) {
        // wait if we are at the head
        await new Promise(resolve => setTimeout(resolve, 1000));
      }

      // Continue query from nextBlock
      query.fromBlock = res.nextBlock;
    }
}

main();
