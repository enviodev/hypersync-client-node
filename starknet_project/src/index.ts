import { Contract, Provider, constants } from "starknet";
import DAI_ABI from "../abis/dai.json";

// NetworkN;
const main = async () => {
  const provider = new Provider({
    sequencer: { network: constants.NetworkName.SN_MAIN },
  });

  const DAI_ADDRESS =
    "0x00da114221cb83fa859dbdb4c44beeaa0bb37c7537ad5ae66fe5e0efd20e6eb3";
  const daiContract = new Contract(DAI_ABI, DAI_ADDRESS, provider);

  let txReceipt: any = await provider.getTransaction(
    "0x0007b32bcee90bdba4449645ac666635ed3de8a58a48e11817567b3b408054c9",
  );

  console.log(txReceipt.events);

  let events = daiContract.parseEvents(txReceipt);

  console.log(events);
};

main();
