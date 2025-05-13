[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / Events

# Interface: Events

## Table of contents

### Properties

- [archiveHeight](Events.md#archiveheight)
- [events](Events.md#events)
- [nextBlock](Events.md#nextblock)
- [rollbackGuard](Events.md#rollbackguard)
- [totalExecutionTime](Events.md#totalexecutiontime)

## Properties

### archiveHeight

• `Optional` **archiveHeight**: `number`

Current height of the source hypersync instance

#### Defined in

[index.d.ts:553](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L553)

___

### events

• **events**: [`Event`](Event.md)[]

Response data

#### Defined in

[index.d.ts:563](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L563)

___

### nextBlock

• **nextBlock**: `number`

Next block to query for, the responses are paginated so,
 the caller should continue the query from this block if they
 didn't get responses up to the to_block they specified in the Query.

#### Defined in

[index.d.ts:559](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L559)

___

### rollbackGuard

• `Optional` **rollbackGuard**: [`RollbackGuard`](RollbackGuard.md)

Rollback guard, supposed to be used to detect rollbacks

#### Defined in

[index.d.ts:565](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L565)

___

### totalExecutionTime

• **totalExecutionTime**: `number`

Total time it took the hypersync instance to execute the query.

#### Defined in

[index.d.ts:561](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L561)
