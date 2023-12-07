import { Contract, Provider, constants } from "starknet";
import { readFile, writeFile } from "fs/promises";

// NetworkN;
const main = async () => {
  const provider = new Provider({
    sequencer: { network: constants.NetworkName.SN_MAIN },
  });
  const testClassHash =
    "0x01cb96b938da26c060d5fd807eef8b580c49490926393a5eeb408a89f84b9b46";
  const { abi } = await provider.getClassByHash(testClassHash);

  await writeFile("./abis/dai.json", JSON.stringify(abi));
};

main();
