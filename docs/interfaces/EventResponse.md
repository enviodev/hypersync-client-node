[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / EventResponse

# Interface: EventResponse

## Table of contents

### Properties

- [archiveHeight](EventResponse.md#archiveheight)
- [data](EventResponse.md#data)
- [nextBlock](EventResponse.md#nextblock)
- [rollbackGuard](EventResponse.md#rollbackguard)
- [totalExecutionTime](EventResponse.md#totalexecutiontime)

## Properties

### archiveHeight

• `Optional` **archiveHeight**: `number`

Current height of the source hypersync instance

#### Defined in

[index.d.ts:537](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L537)

___

### data

• **data**: [`Event`](Event.md)[]

Response data

#### Defined in

[index.d.ts:547](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L547)

___

### nextBlock

• **nextBlock**: `number`

Next block to query for, the responses are paginated so,
 the caller should continue the query from this block if they
 didn't get responses up to the to_block they specified in the Query.

#### Defined in

[index.d.ts:543](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L543)

___

### rollbackGuard

• `Optional` **rollbackGuard**: [`RollbackGuard`](RollbackGuard.md)

Rollback guard, supposed to be used to detect rollbacks

#### Defined in

[index.d.ts:549](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L549)

___

### totalExecutionTime

• **totalExecutionTime**: `number`

Total time it took the hypersync instance to execute the query.

#### Defined in

[index.d.ts:545](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L545)
