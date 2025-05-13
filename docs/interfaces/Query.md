[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / Query

# Interface: Query

## Table of contents

### Properties

- [blocks](Query.md#blocks)
- [fieldSelection](Query.md#fieldselection)
- [fromBlock](Query.md#fromblock)
- [includeAllBlocks](Query.md#includeallblocks)
- [joinMode](Query.md#joinmode)
- [logs](Query.md#logs)
- [maxNumBlocks](Query.md#maxnumblocks)
- [maxNumLogs](Query.md#maxnumlogs)
- [maxNumTraces](Query.md#maxnumtraces)
- [maxNumTransactions](Query.md#maxnumtransactions)
- [toBlock](Query.md#toblock)
- [traces](Query.md#traces)
- [transactions](Query.md#transactions)

## Properties

### blocks

• `Optional` **blocks**: [`BlockSelection`](BlockSelection.md)[]

List of block selections, the query will return blocks that match any of these selections

#### Defined in

[index.d.ts:280](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L280)

___

### fieldSelection

• **fieldSelection**: [`FieldSelection`](FieldSelection.md)

Field selection. The user can select which fields they are interested in, requesting less fields will improve
 query execution time and reduce the payload size so the user should always use a minimal number of fields.

#### Defined in

[index.d.ts:291](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L291)

___

### fromBlock

• **fromBlock**: `number`

The block to start the query from

#### Defined in

[index.d.ts:253](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L253)

___

### includeAllBlocks

• `Optional` **includeAllBlocks**: `boolean`

Weather to include all blocks regardless of if they are related to a returned transaction or log. Normally
 the server will return only the blocks that are related to the transaction or logs in the response. But if this
 is set to true, the server will return data for all blocks in the requested range [from_block, to_block).

#### Defined in

[index.d.ts:286](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L286)

___

### joinMode

• `Optional` **joinMode**: [`JoinMode`](../enums/JoinMode.md)

Selects join mode for the query,
Default: join in this order logs -> transactions -> traces -> blocks
JoinAll: join everything to everything. For example if logSelection matches log0, we get the
associated transaction of log0 and then we get associated logs of that transaction as well. Applites similarly
to blocks, traces.
JoinNothing: join nothing.

#### Defined in

[index.d.ts:320](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L320)

___

### logs

• `Optional` **logs**: [`LogSelection`](LogSelection.md)[]

List of log selections, these have an or relationship between them, so the query will return logs
that match any of these selections.

#### Defined in

[index.d.ts:268](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L268)

___

### maxNumBlocks

• `Optional` **maxNumBlocks**: `number`

Maximum number of blocks that should be returned, the server might return more blocks than this number but
 it won't overshoot by too much.

#### Defined in

[index.d.ts:296](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L296)

___

### maxNumLogs

• `Optional` **maxNumLogs**: `number`

Maximum number of logs that should be returned, the server might return more logs than this number but
 it won't overshoot by too much.

#### Defined in

[index.d.ts:306](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L306)

___

### maxNumTraces

• `Optional` **maxNumTraces**: `number`

Maximum number of traces that should be returned, the server might return more traces than this number but
 it won't overshoot by too much.

#### Defined in

[index.d.ts:311](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L311)

___

### maxNumTransactions

• `Optional` **maxNumTransactions**: `number`

Maximum number of transactions that should be returned, the server might return more transactions than this number but
 it won't overshoot by too much.

#### Defined in

[index.d.ts:301](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L301)

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

[index.d.ts:263](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L263)

___

### traces

• `Optional` **traces**: [`TraceSelection`](TraceSelection.md)[]

List of trace selections, the query will return traces that match any of these selections and
 it will re turn traces that are related to the returned logs.

#### Defined in

[index.d.ts:278](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L278)

___

### transactions

• `Optional` **transactions**: [`TransactionSelection`](TransactionSelection.md)[]

List of transaction selections, the query will return transactions that match any of these selections and
 it will return transactions that are related to the returned logs.

#### Defined in

[index.d.ts:273](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L273)
