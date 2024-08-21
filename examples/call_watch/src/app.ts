import {HypersyncClient, Decoder, LogField} from "@envio-dev/hypersync-client";
import fs from "node:fs";
import {CallDecoder, TransactionField} from "../../../index";

const DAI_ADDRESS = "0x6B175474E89094C44Da98b954EedeAC495271d0F";

async function main() {
    // Create hypersync client using the mainnet hypersync endpoint
    const client = HypersyncClient.new({
        url: "https://eth.hypersync.xyz"
    });

    // The query to run
    const query = {
        // start from tip of the chain
        "fromBlock": 20500000,
        "transactions": [
            {
                "from": [DAI_ADDRESS]
            },
            {
                "to": [DAI_ADDRESS]
            }
        ],
        // Select the fields we are interested in, notice topics are selected as topic0,1,2,3
        "fieldSelection": {
            "transaction": [
                TransactionField.Hash,
                TransactionField.Input,
            ]
        },
    };

    const decoder = CallDecoder.fromSignatures([
        "transfer(address dst, uint256 wad)",
    ]);


    while (true) {
        const res = await client.get(query);
        if (res.data.transactions.length !== 0) {
            // Decode the log on a background thread so we don't block the event loop.
            // Can also use decoder.decodeLogsSync if it is more convenient.
            const decodedInputs = await decoder.decodeTransactionsInput(res.data.transactions);
            for (const input of decodedInputs) {
                if (input === null) {
                    continue;
                }
                console.log(`Transaction decoded. Addr ${input[0].val}, Wad ${input[1].val}`);
            }
        } else {
            console.log(`no tx`);
        }
        let height = res.archiveHeight;
        while (height < res.nextBlock) {
            // wait if we are at the head
            console.log(`waiting for chain to advance. Height is ${height}`);
            height = await client.getHeight();
            await new Promise(resolve => setTimeout(resolve, 1000));
        }

        // Continue query from nextBlock
        query.fromBlock = res.nextBlock;
    }
}

main();
