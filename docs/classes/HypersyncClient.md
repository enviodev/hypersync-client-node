[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / HypersyncClient

# Class: HypersyncClient

## Table of contents

### Constructors

- [constructor](HypersyncClient.md#constructor)

### Methods

- [createParquetFolder](HypersyncClient.md#createparquetfolder)
- [getHeight](HypersyncClient.md#getheight)
- [sendEventsReq](HypersyncClient.md#sendeventsreq)
- [sendReq](HypersyncClient.md#sendreq)
- [new](HypersyncClient.md#new)

## Constructors

### constructor

• **new HypersyncClient**(): [`HypersyncClient`](HypersyncClient.md)

#### Returns

[`HypersyncClient`](HypersyncClient.md)

## Methods

### createParquetFolder

▸ **createParquetFolder**(`query`, `path`): `Promise`\<`void`\>

Create a parquet file by executing a query.

If the query can't be finished in a single request, this function will
 keep on making requests using the pagination mechanism (next_block) until
 it reaches the end. It will stream data into the parquet file as it comes from
. the server.

Path should point to a folder that will contain the parquet files in the end.

#### Parameters

| Name | Type |
| :------ | :------ |
| `query` | [`Query`](../interfaces/Query.md) |
| `path` | `string` |

#### Returns

`Promise`\<`void`\>

#### Defined in

[index.d.ts:244](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L244)

___

### getHeight

▸ **getHeight**(): `Promise`\<`number`\>

Get the height of the source hypersync instance

#### Returns

`Promise`\<`number`\>

#### Defined in

[index.d.ts:233](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L233)

___

### sendEventsReq

▸ **sendEventsReq**(`query`): `Promise`\<[`Events`](../interfaces/Events.md)\>

Send a event query request to the source hypersync instance.

This executes the same query as send_req function on the source side but
it groups data for each event(log) so it is easier to process it.

#### Parameters

| Name | Type |
| :------ | :------ |
| `query` | [`Query`](../interfaces/Query.md) |

#### Returns

`Promise`\<[`Events`](../interfaces/Events.md)\>

#### Defined in

[index.d.ts:257](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L257)

___

### sendReq

▸ **sendReq**(`query`): `Promise`\<[`QueryResponse`](../interfaces/QueryResponse.md)\>

Send a query request to the source hypersync instance.

Returns a query response which contains block, tx and log data.

#### Parameters

| Name | Type |
| :------ | :------ |
| `query` | [`Query`](../interfaces/Query.md) |

#### Returns

`Promise`\<[`QueryResponse`](../interfaces/QueryResponse.md)\>

#### Defined in

[index.d.ts:250](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L250)

___

### new

▸ **new**(`cfg`): [`HypersyncClient`](HypersyncClient.md)

Create a new client with given config

#### Parameters

| Name | Type |
| :------ | :------ |
| `cfg` | [`Config`](../interfaces/Config.md) |

#### Returns

[`HypersyncClient`](HypersyncClient.md)

#### Defined in

[index.d.ts:231](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L231)
