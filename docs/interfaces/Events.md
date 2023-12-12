[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / Events

# Interface: Events

## Table of contents

### Properties

- [archiveHeight](Events.md#archiveheight)
- [events](Events.md#events)
- [nextBlock](Events.md#nextblock)
- [totalExecutionTime](Events.md#totalexecutiontime)

## Properties

### archiveHeight

• `Optional` **archiveHeight**: `number`

Current height of the source hypersync instance

#### Defined in

[index.d.ts:210](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L210)

___

### events

• **events**: [`Event`](Event.md)[]

Response data

#### Defined in

[index.d.ts:220](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L220)

___

### nextBlock

• **nextBlock**: `number`

Next block to query for, the responses are paginated so,
 the caller should continue the query from this block if they
 didn't get responses up to the to_block they specified in the Query.

#### Defined in

[index.d.ts:216](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L216)

___

### totalExecutionTime

• **totalExecutionTime**: `number`

Total time it took the hypersync instance to execute the query.

#### Defined in

[index.d.ts:218](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L218)
