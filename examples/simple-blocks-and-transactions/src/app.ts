import { HypersyncClient, presetQueryBlocksAndTransactions } from "@envio-dev/hypersync-client";

async function main() {
  // Create hypersync client using the mainnet hypersync endpoint
  const client = HypersyncClient.new({
    url: "https://eth.hypersync.xyz"
  });

  // query is inclusive of from_block, exclusive of to_block so this will return 49 blocks
  let query = presetQueryBlocksAndTransactions(17_000_000, 17_000_050);

  console.log("Running the query...");

  // Run the query once, the query is automatically paginated so it will return when it reaches some limit (time, response size etc.)
  //  there is a nextBlock field on the response object so we can set the fromBlock of our query to this value and continue our query until
  // res.nextBlock is equal to res.archiveHeight or query.toBlock in case we specified an end block.
  const res = await client.get(query);

  console.log(`Query returned ${res.data.blocks.length} blocks and ${res.data.transactions.length} transactions`)
}

main();
