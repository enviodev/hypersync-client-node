import {
  HypersyncClient,
  Decoder,
  BlockField,
  LogField,
  TransactionField,
  Query,
} from "@envio-dev/hypersync-client";

async function main() {
  // Create hypersync client using the mainnet hypersync endpoint
  // Passing null config makes it use default
  const client = HypersyncClient.new({
    url: "https://8453.hypersync.xyz",
  });

  // The query to run
  const query: Query = {
    // fromBlock: 25225492,
    // toBlock: 25300182,
    fromBlock: 25236222,
    toBlock: 25236223,
    logs: [
      {
        address: [
          "0xeccfae0f0f6a6696547e645ae00ff99619d47143",
          "0x613940bff09917cf91ee669c306c3de8d3d081fe",
        ],
        topics: [
          [
            "0x98636036cb66a9c19a37435efc1e90142190214e8abeb821bdba3f2990dd4c95",
            "0x7a53080ba414158be7ec69b987b5fb7d07dee101fe85488f0853ae16239d0bde",
            "0x70935338e69775456a85ddef226c395fb668b63fa0115f5f20610b388e6ca9c0",
            "0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67",
            "0x596b573906218d3411850b26a6b437d6c4522fdb43d2d2386263f86d50b8b151",
          ],
          [],
          [],
          [],
        ],
      },
    ],
    fieldSelection: {
      block: [BlockField.Number, BlockField.Timestamp, BlockField.Hash],
      transaction: [],
      log: [
        LogField.Address,
        LogField.Data,
        LogField.LogIndex,
        LogField.BlockNumber,
        LogField.Topic0,
        LogField.Topic1,
        LogField.Topic2,
        LogField.Topic3,
      ],
    },
  };

  console.log("Running the query...");

  // Run the query once, the query is automatically paginated so it will return when it reaches some limit (time, response size etc.)
  //  there is a nextBlock field on the response object so we can set the fromBlock of our query to this value and continue our query until
  // res.nextBlock is equal to res.archiveHeight or query.toBlock in case we specified an end block.
  const res = await client.get(query);

  console.log(`Ran the query once. Next block to query is ${res.nextBlock}`);

  // const totalBlocks = res.nextBlock - query.fromBlock;

  // console.log(
  //   `Total events was ${res.data.logs.length} in ${totalBlocks} blocks in ${res.data.logs.length} transfers.`
  // );
  console.log(res.data.logs.filter((e) => e.blockNumber === 25236222));
  console.log("Evetns n:",res.data.logs.length);
}

main();
