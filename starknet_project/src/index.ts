import { Contract, Provider, constants, hash, num } from "starknet";
import DAI_ABI from "../abis/dai.json";
import { HypersyncClient, Decoder } from "hypersync";
import { ethers } from "ethers";


// NetworkN;
const main = async () => {
  const client = await HypersyncClient.new({
    url: "http://localhost:2104/"
  });


  const DAI_ADDRESS =
    "0x00da114221cb83fa859dbdb4c44beeaa0bb37c7537ad5ae66fe5e0efd20e6eb3";
  const TRANSFER = "Transfer"
  let felt = num.toHex(hash.starknetKeccak(TRANSFER))

  const transfer_name_hex = ethers.toBeHex(hash.starknetKeccak(TRANSFER), 32)
  console.log(transfer_name_hex)
  let felt_as_hash = "0x00" + felt.substring(2)
  console.log(felt_as_hash)

  // const dai_address_hex = ethers.toBeHex(num.toBigInt(DAI_ADDRESS), 20)

  const query = {
    // start from block 0 and go to the end of the chain (we don't specify a toBlock).
    "fromBlock": 2365,
    "toBlock": 2366,
    // The logs we want. We will also automatically get transactions and blocks relating to these logs (the query implicitly joins them).
    "logs": [

      {
        "topics": [
          [transfer_name_hex]
        ]
      },

    ],
    "transactions": [{}],
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
    }
  };

  let resp = await client.sendReqStarknet(query);
  console.log(JSON.stringify(resp));

  // await writeFile("./out.json", JSON.stringify(resp));

  // await client.createParquetFolder(query, "data");
  // console.log("finished writing parquet folder");

  const provider = new Provider({
    sequencer: { network: constants.NetworkName.SN_MAIN },
  });


  const daiContract = new Contract(DAI_ABI, DAI_ADDRESS, provider);

  let txReceipt: any = await provider.getTransaction(
    "0x0007b32bcee90bdba4449645ac666635ed3de8a58a48e11817567b3b408054c9",
  );

  console.log(txReceipt.events);

  let events = daiContract.parseEvents(txReceipt);

  console.log(events);
};

main();
