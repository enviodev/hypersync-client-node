# hypersync-client-node

[![npm](https://img.shields.io/npm/v/@envio-dev/hypersync-client)](https://www.npmjs.com/package/@envio-dev/hypersync-client) [![npm downloads](https://img.shields.io/npm/dm/@envio-dev/hypersync-client)](https://www.npmjs.com/package/@envio-dev/hypersync-client) [![Discord](https://img.shields.io/badge/Discord-Join%20Chat-7289da?logo=discord&logoColor=white)](https://discord.gg/Q9qt8gZ2fX)

Node.js client for [Envio's](https://envio.dev) HyperSync. TypeScript-first, built on top of the Rust implementation via NAPI bindings for high-performance blockchain data access.

## What is HyperSync?

[HyperSync](https://docs.envio.dev/docs/HyperSync/overview) is Envio's high-performance blockchain data retrieval layer. It is a purpose-built alternative to JSON-RPC endpoints, offering up to 2000x faster data access across 70+ EVM-compatible networks and Fuel.

HyperSync lets you query logs, transactions, blocks, and traces with flexible filtering and field selection, returning only the data you need.

## Features

- **TypeScript-first**: Full TypeScript types and IntelliSense support
- **High performance**: Built on a Rust core via NAPI bindings
- **Binary transport**: Optimized serialization to minimize bandwidth and maximize throughput
- **Flexible queries**: Filter logs, transactions, blocks, and traces
- **Field selection**: Choose exactly which fields to return
- **Preset queries**: Built-in helpers for common query patterns
- **Parquet export**: Stream data directly to Parquet files
- **Streaming**: Process large datasets without loading everything into memory
- **70+ networks**: Access any [HyperSync-supported network](https://docs.envio.dev/docs/HyperSync/hypersync-supported-networks)

## Installation

```bash
# npm
npm install @envio-dev/hypersync-client

# pnpm
pnpm add @envio-dev/hypersync-client

# yarn
yarn add @envio-dev/hypersync-client
```

## API Token

An API token is required to use HyperSync. [Get your token here](https://docs.envio.dev/docs/HyperSync/api-tokens), then set it as an environment variable:

```bash
export ENVIO_API_TOKEN="your-token-here"
```

## Quick Start

Fetch Transfer event logs from a USDT contract on Ethereum:

```typescript
import { HypersyncClient, presetQueryLogsOfEvent } from "@envio-dev/hypersync-client";

async function main() {
  const client = new HypersyncClient({
    url: "https://eth.hypersync.xyz",
    apiToken: process.env.ENVIO_API_TOKEN!,
  });

  const usdtContract = "0xdAC17F958D2ee523a2206206994597C13D831ec7";

  // ERC-20 Transfer event topic0
  const transferTopic = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

  const query = presetQueryLogsOfEvent(usdtContract, transferTopic, 17_000_000, 17_000_050);

  const res = await client.get(query);
  console.log(`Found ${res.data.logs.length} Transfer events`);
}

main();
```

See the [examples directory](./examples) for more patterns including block data, wallet transactions, Parquet export, and real-time streaming.

## Connecting to Different Networks

Change the `url` to connect to any supported network:

```typescript
// Arbitrum
const client = new HypersyncClient({
  url: "https://arbitrum.hypersync.xyz",
  apiToken: process.env.ENVIO_API_TOKEN!,
});

// Base
const client = new HypersyncClient({
  url: "https://base.hypersync.xyz",
  apiToken: process.env.ENVIO_API_TOKEN!,
});
```

See the full list of [supported networks and URLs](https://docs.envio.dev/docs/HyperSync/hypersync-supported-networks).

## Documentation

- [API Documentation](https://enviodev.github.io/hypersync-client-node/)
- [HyperSync Documentation](https://docs.envio.dev/docs/HyperSync/overview)
- [Query Reference](https://docs.envio.dev/docs/HyperSync/hypersync-query)
- [All Client Libraries](https://docs.envio.dev/docs/HyperSync/hypersync-clients) (Python, Rust, Go)
- [npm Package](https://www.npmjs.com/package/@envio-dev/hypersync-client)

## FAQ

**How does this compare to ethers.js or viem for data fetching?**
HyperSync retrieves data up to 2000x faster than traditional JSON-RPC. It is designed for bulk historical data access, not transaction signing or contract writes. Use HyperSync alongside ethers.js or viem for read-heavy applications.

**Do I need an API token?**
Yes, an API token is required. [Get one here](https://docs.envio.dev/docs/HyperSync/api-tokens).

**Which networks are supported?**
70+ EVM-compatible networks and Fuel. See the [full list](https://docs.envio.dev/docs/HyperSync/hypersync-supported-networks).

**Can I export data to Parquet?**
Yes. See `examples/parquet-out` for an example of streaming data to a Parquet file.

**Does this work with Deno or Bun?**
The package targets Node.js via NAPI bindings. Bun is broadly compatible with Node.js native modules. Deno compatibility may vary.

**How is this different from the Rust client?**
This client is built on top of the [Rust client](https://github.com/enviodev/hypersync-client-rust) via NAPI bindings. It provides a TypeScript-first interface for JavaScript/Node.js developers.

## Support

- [Discord community](https://discord.gg/Q9qt8gZ2fX)
- [GitHub Issues](https://github.com/enviodev/hypersync-client-node/issues)
- [Documentation](https://docs.envio.dev/docs/HyperSync/overview)
