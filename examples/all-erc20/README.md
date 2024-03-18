# Index all erc20 on the whole of Ethereum

Example of using hypersync-client to get data for all erc20 transfers.

The example uses the 'Transfer' event and counts all the amounts in the first batch of events that it recieves. The script doesn't scan the entire Ethereum chain, but rather goes through as many events as it can in the initial returned range.

## Prerequisites

- Node.js (version 18.0.0 or above - rather stick to even/lts releases)
- npm/yarn/pnpm

## Run

```bash
git clone https://github.com/enviodev/hypersync-client-node.github
cd hypersync-client-node/examples/all-erc20
npm install
npm build
npm start
```

## Comments

The code is well commented, so best you go through that directly. Things to note, this is just an example of the API, and not an accurate example. So it doesn't take into account different decimals of ERC20 tokens, it only runs the query once - so it only does the first ~1.3 million blocks, it also includes volume of intra-contract-interaction transfers which might not be considered volume.

So areas that could be explored further in pursuit of learning how to use Hypersync:

- Scan the full chain by repeating queries from the end block rage all the way until the head.
- Scan only erc20 contracts you care about by including the contract addresses in the query.
- Get the decimals of the erc20 tokens and convert the amounts to a standard unit (might not be completely feasible if unless you are doing a subset of erc20 tokens).
- Analyze only the resultant change in erc20 balance in a transaction, and exclude for example if multiple intra-contract transfers happen. This could be a fun one to work out.
