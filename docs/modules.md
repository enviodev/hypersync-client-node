[@envio-dev/hypersync-client](README.md) / Exports

# @envio-dev/hypersync-client

## Table of contents

### Enumerations

- [BlockField](enums/BlockField.md)
- [DataType](enums/DataType.md)
- [HexOutput](enums/HexOutput.md)
- [JoinMode](enums/JoinMode.md)
- [LogField](enums/LogField.md)
- [TraceField](enums/TraceField.md)
- [TransactionField](enums/TransactionField.md)

### Classes

- [CallDecoder](classes/CallDecoder.md)
- [Decoder](classes/Decoder.md)
- [EventStream](classes/EventStream.md)
- [HypersyncClient](classes/HypersyncClient.md)
- [QueryResponseStream](classes/QueryResponseStream.md)

### Interfaces

- [AccessList](interfaces/AccessList.md)
- [Authorization](interfaces/Authorization.md)
- [AuthorizationSelection](interfaces/AuthorizationSelection.md)
- [Block](interfaces/Block.md)
- [BlockSelection](interfaces/BlockSelection.md)
- [ClientConfig](interfaces/ClientConfig.md)
- [ColumnMapping](interfaces/ColumnMapping.md)
- [DecodedEvent](interfaces/DecodedEvent.md)
- [DecodedSolValue](interfaces/DecodedSolValue.md)
- [Event](interfaces/Event.md)
- [EventResponse](interfaces/EventResponse.md)
- [Events](interfaces/Events.md)
- [FieldSelection](interfaces/FieldSelection.md)
- [Log](interfaces/Log.md)
- [LogSelection](interfaces/LogSelection.md)
- [Query](interfaces/Query.md)
- [QueryResponse](interfaces/QueryResponse.md)
- [QueryResponseData](interfaces/QueryResponseData.md)
- [RollbackGuard](interfaces/RollbackGuard.md)
- [StreamConfig](interfaces/StreamConfig.md)
- [Trace](interfaces/Trace.md)
- [TraceSelection](interfaces/TraceSelection.md)
- [Transaction](interfaces/Transaction.md)
- [TransactionSelection](interfaces/TransactionSelection.md)
- [Withdrawal](interfaces/Withdrawal.md)

### Functions

- [presetQueryBlocksAndTransactionHashes](modules.md#presetqueryblocksandtransactionhashes)
- [presetQueryBlocksAndTransactions](modules.md#presetqueryblocksandtransactions)
- [presetQueryLogs](modules.md#presetquerylogs)
- [presetQueryLogsOfEvent](modules.md#presetquerylogsofevent)

## Functions

### presetQueryBlocksAndTransactionHashes

▸ **presetQueryBlocksAndTransactionHashes**(`fromBlock`, `toBlock?`): [`Query`](interfaces/Query.md)

Returns a query object for all Blocks and hashes of the Transactions within the block range
(from_block, to_block].  Also returns the block_hash and block_number fields on each Transaction
so it can be mapped to a block.  If to_block is None then query runs to the head of the chain.

#### Parameters

| Name | Type |
| :------ | :------ |
| `fromBlock` | `number` |
| `toBlock?` | `number` |

#### Returns

[`Query`](interfaces/Query.md)

#### Defined in

[index.d.ts:62](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L62)

___

### presetQueryBlocksAndTransactions

▸ **presetQueryBlocksAndTransactions**(`fromBlock`, `toBlock?`): [`Query`](interfaces/Query.md)

Returns a query for all Blocks and Transactions within the block range (from_block, to_block]
If to_block is None then query runs to the head of the chain.

#### Parameters

| Name | Type |
| :------ | :------ |
| `fromBlock` | `number` |
| `toBlock?` | `number` |

#### Returns

[`Query`](interfaces/Query.md)

#### Defined in

[index.d.ts:56](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L56)

___

### presetQueryLogs

▸ **presetQueryLogs**(`contractAddress`, `fromBlock`, `toBlock?`): [`Query`](interfaces/Query.md)

Returns a query object for all Logs within the block range from the given address.
If to_block is None then query runs to the head of the chain.

#### Parameters

| Name | Type |
| :------ | :------ |
| `contractAddress` | `string` |
| `fromBlock` | `number` |
| `toBlock?` | `number` |

#### Returns

[`Query`](interfaces/Query.md)

#### Defined in

[index.d.ts:67](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L67)

___

### presetQueryLogsOfEvent

▸ **presetQueryLogsOfEvent**(`contractAddress`, `topic0`, `fromBlock`, `toBlock?`): [`Query`](interfaces/Query.md)

Returns a query for all Logs within the block range from the given address with a
matching topic0 event signature.  Topic0 is the keccak256 hash of the event signature.
If to_block is None then query runs to the head of the chain.

#### Parameters

| Name | Type |
| :------ | :------ |
| `contractAddress` | `string` |
| `topic0` | `string` |
| `fromBlock` | `number` |
| `toBlock?` | `number` |

#### Returns

[`Query`](interfaces/Query.md)

#### Defined in

[index.d.ts:73](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L73)
