[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / QueryResponse

# Interface: QueryResponse

## Table of contents

### Properties

- [archiveHeight](QueryResponse.md#archiveheight)
- [data](QueryResponse.md#data)
- [nextBlock](QueryResponse.md#nextblock)
- [totalExecutionTime](QueryResponse.md#totalexecutiontime)

## Properties

### archiveHeight

• `Optional` **archiveHeight**: `number`

Current height of the source hypersync instance

#### Defined in

[index.d.ts:196](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L196)

___

### data

• **data**: [`QueryResponseData`](QueryResponseData.md)

Response data

#### Defined in

[index.d.ts:206](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L206)

___

### nextBlock

• **nextBlock**: `number`

Next block to query for, the responses are paginated so,
 the caller should continue the query from this block if they
 didn't get responses up to the to_block they specified in the Query.

#### Defined in

[index.d.ts:202](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L202)

___

### totalExecutionTime

• **totalExecutionTime**: `number`

Total time it took the hypersync instance to execute the query.

#### Defined in

[index.d.ts:204](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L204)
