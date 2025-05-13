[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / HypersyncClient

# Class: HypersyncClient

## Table of contents

### Constructors

- [constructor](HypersyncClient.md#constructor)

### Methods

- [collect](HypersyncClient.md#collect)
- [collectEvents](HypersyncClient.md#collectevents)
- [collectParquet](HypersyncClient.md#collectparquet)
- [get](HypersyncClient.md#get)
- [getChainId](HypersyncClient.md#getchainid)
- [getEvents](HypersyncClient.md#getevents)
- [getHeight](HypersyncClient.md#getheight)
- [stream](HypersyncClient.md#stream)
- [streamEvents](HypersyncClient.md#streamevents)
- [new](HypersyncClient.md#new)

## Constructors

### constructor

• **new HypersyncClient**(): [`HypersyncClient`](HypersyncClient.md)

#### Returns

[`HypersyncClient`](HypersyncClient.md)

## Methods

### collect

▸ **collect**(`query`, `config`): `Promise`\<[`QueryResponse`](../interfaces/QueryResponse.md)\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `query` | [`Query`](../interfaces/Query.md) |
| `config` | [`StreamConfig`](../interfaces/StreamConfig.md) |

#### Returns

`Promise`\<[`QueryResponse`](../interfaces/QueryResponse.md)\>

#### Defined in

[index.d.ts:595](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L595)

___

### collectEvents

▸ **collectEvents**(`query`, `config`): `Promise`\<[`EventResponse`](../interfaces/EventResponse.md)\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `query` | [`Query`](../interfaces/Query.md) |
| `config` | [`StreamConfig`](../interfaces/StreamConfig.md) |

#### Returns

`Promise`\<[`EventResponse`](../interfaces/EventResponse.md)\>

#### Defined in

[index.d.ts:596](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L596)

___

### collectParquet

▸ **collectParquet**(`path`, `query`, `config`): `Promise`\<`void`\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `path` | `string` |
| `query` | [`Query`](../interfaces/Query.md) |
| `config` | [`StreamConfig`](../interfaces/StreamConfig.md) |

#### Returns

`Promise`\<`void`\>

#### Defined in

[index.d.ts:597](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L597)

___

### get

▸ **get**(`query`): `Promise`\<[`QueryResponse`](../interfaces/QueryResponse.md)\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `query` | [`Query`](../interfaces/Query.md) |

#### Returns

`Promise`\<[`QueryResponse`](../interfaces/QueryResponse.md)\>

#### Defined in

[index.d.ts:598](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L598)

___

### getChainId

▸ **getChainId**(): `Promise`\<`number`\>

Get the chain_id of the source hypersync instance

#### Returns

`Promise`\<`number`\>

#### Defined in

[index.d.ts:594](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L594)

___

### getEvents

▸ **getEvents**(`query`): `Promise`\<[`EventResponse`](../interfaces/EventResponse.md)\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `query` | [`Query`](../interfaces/Query.md) |

#### Returns

`Promise`\<[`EventResponse`](../interfaces/EventResponse.md)\>

#### Defined in

[index.d.ts:599](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L599)

___

### getHeight

▸ **getHeight**(): `Promise`\<`number`\>

Get the height of the source hypersync instance

#### Returns

`Promise`\<`number`\>

#### Defined in

[index.d.ts:592](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L592)

___

### stream

▸ **stream**(`query`, `config`): `Promise`\<[`QueryResponseStream`](QueryResponseStream.md)\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `query` | [`Query`](../interfaces/Query.md) |
| `config` | [`StreamConfig`](../interfaces/StreamConfig.md) |

#### Returns

`Promise`\<[`QueryResponseStream`](QueryResponseStream.md)\>

#### Defined in

[index.d.ts:600](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L600)

___

### streamEvents

▸ **streamEvents**(`query`, `config`): `Promise`\<[`EventStream`](EventStream.md)\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `query` | [`Query`](../interfaces/Query.md) |
| `config` | [`StreamConfig`](../interfaces/StreamConfig.md) |

#### Returns

`Promise`\<[`EventStream`](EventStream.md)\>

#### Defined in

[index.d.ts:601](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L601)

___

### new

▸ **new**(`cfg?`): [`HypersyncClient`](HypersyncClient.md)

Create a new client with given config

#### Parameters

| Name | Type |
| :------ | :------ |
| `cfg?` | [`ClientConfig`](../interfaces/ClientConfig.md) |

#### Returns

[`HypersyncClient`](HypersyncClient.md)

#### Defined in

[index.d.ts:590](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L590)
