import { HypersyncClient, type Query } from "@envio-dev/hypersync-client";

async function main() {
  // Create hypersync client using the mainnet hypersync endpoint
  const client = new HypersyncClient({
    url: "https://eth.hypersync.xyz",
    apiToken: process.env.ENVIO_API_TOKEN!,
  });

  // The query to run
  const query: Query = {
    "fromBlock": 0,
    "transactions": [
      // get all transactions coming from and going to our address.
      {
        from: ["0x5a830d7a5149b2f1a2e72d15cd51b84379ee81e5"]
      },
      {
        to: ["0x5a830d7a5149b2f1a2e72d15cd51b84379ee81e5"]
      }
    ],
    "fieldSelection": {
      "transaction": [
        "BlockNumber",
        "Hash",
        "From",
        "To",
        "Value",
      ]
    }
  };

  // Stream data in reverse order
  //
  // This will parallelize internal requests so we don't have to worry about pipelining/parallelizing make request -> handle response -> handle data loop
  const receiver = await client.stream(query, { reverse: true });

  while (true) {
    let res = await receiver.recv();
    if (res === null) {
      break;
    }
    for (const tx of res.data.transactions) {
      console.log(tx);
    }
  }
}

main();
