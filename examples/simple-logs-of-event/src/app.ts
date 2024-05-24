import { HypersyncClient, presetQueryLogsOfEvent } from "@envio-dev/hypersync-client";

async function main() {
  // Create hypersync client using the mainnet hypersync endpoint
  const client = HypersyncClient.new({
    url: "https://eth.hypersync.xyz"
  });

  // address to get logs from
  const usdt_contract = "0xdAC17F958D2ee523a2206206994597C13D831ec7";

  // topic0 of transaction event signature (hash of event signature)
  // query will return logs of this event
  const event_topic_0 = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

  // query is inclusive of from_block, exclusive of to_block so this will return 49 blocks
  let query = presetQueryLogsOfEvent(usdt_contract, event_topic_0, 17_000_000, 17_000_050);

  console.log("Running the query...");

  // Run the query once, the query is automatically paginated so it will return when it reaches some limit (time, response size etc.)
  //  there is a nextBlock field on the response object so we can set the fromBlock of our query to this value and continue our query until
  // res.nextBlock is equal to res.archiveHeight or query.toBlock in case we specified an end block.
  const res = await client.get(query);

  console.log(`Query returned ${res.data.logs.length} logs of transfer events from contract ${usdt_contract}`)

}

main();
