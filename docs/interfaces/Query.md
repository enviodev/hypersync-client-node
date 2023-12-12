[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / Query

# Interface: Query

## Table of contents

### Properties

- [fieldSelection](Query.md#fieldselection)
- [fromBlock](Query.md#fromblock)
- [includeAllBlocks](Query.md#includeallblocks)
- [logs](Query.md#logs)
- [maxNumBlocks](Query.md#maxnumblocks)
- [maxNumLogs](Query.md#maxnumlogs)
- [maxNumTransactions](Query.md#maxnumtransactions)
- [toBlock](Query.md#toblock)
- [transactions](Query.md#transactions)

## Properties

### fieldSelection

• **fieldSelection**: [`FieldSelection`](FieldSelection.md)

Field selection. The user can select which fields they are interested in, requesting less fields will improve
 query execution time and reduce the payload size so the user should always use a minimal number of fields.

#### Defined in

[index.d.ts:82](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L82)

___

### fromBlock

• **fromBlock**: `number`

The block to start the query from

#### Defined in

[index.d.ts:51](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L51)

___

### includeAllBlocks

• `Optional` **includeAllBlocks**: `boolean`

Weather to include all blocks regardless of if they are related to a returned transaction or log. Normally
 the server will return only the blocks that are related to the transaction or logs in the response. But if this
 is set to true, the server will return data for all blocks in the requested range [from_block, to_block).

#### Defined in

[index.d.ts:77](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L77)

___

### logs

• `Optional` **logs**: [`LogSelection`](LogSelection.md)[]

List of log selections, these have an or relationship between them, so the query will return logs
that match any of these selections.

#### Defined in

[index.d.ts:66](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L66)

___

### maxNumBlocks

• `Optional` **maxNumBlocks**: `number`

Maximum number of blocks that should be returned, the server might return more blocks than this number but
 it won't overshoot by too much.

#### Defined in

[index.d.ts:87](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L87)

___

### maxNumLogs

• `Optional` **maxNumLogs**: `number`

Maximum number of logs that should be returned, the server might return more logs than this number but
 it won't overshoot by too much.

#### Defined in

[index.d.ts:97](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L97)

___

### maxNumTransactions

• `Optional` **maxNumTransactions**: `number`

Maximum number of transactions that should be returned, the server might return more transactions than this number but
 it won't overshoot by too much.

#### Defined in

[index.d.ts:92](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L92)

___

### toBlock

• `Optional` **toBlock**: `number`

The block to end the query at. If not specified, the query will go until the
 end of data. Exclusive, the returned range will be [from_block..to_block).

The query will return before it reaches this target block if it hits the time limit
 configured on the server. The user should continue their query by putting the
 next_block field in the response into from_block field of their next query. This implements
 pagination.

#### Defined in

[index.d.ts:61](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L61)

___

### transactions

• `Optional` **transactions**: [`TransactionSelection`](TransactionSelection.md)[]

List of transaction selections, the query will return transactions that match any of these selections and
 it will return transactions that are related to the returned logs.

#### Defined in

[index.d.ts:71](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L71)
