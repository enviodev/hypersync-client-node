[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / Decoder

# Class: Decoder

## Table of contents

### Constructors

- [constructor](Decoder.md#constructor)

### Methods

- [decodeEvents](Decoder.md#decodeevents)
- [decodeEventsSync](Decoder.md#decodeeventssync)
- [decodeLogs](Decoder.md#decodelogs)
- [decodeLogsSync](Decoder.md#decodelogssync)
- [new](Decoder.md#new)

## Constructors

### constructor

• **new Decoder**(): [`Decoder`](Decoder.md)

#### Returns

[`Decoder`](Decoder.md)

## Methods

### decodeEvents

▸ **decodeEvents**(`events`): `Promise`\<[`DecodedEvent`](../interfaces/DecodedEvent.md)[]\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `events` | [`Event`](../interfaces/Event.md)[] |

#### Returns

`Promise`\<[`DecodedEvent`](../interfaces/DecodedEvent.md)[]\>

#### Defined in

[index.d.ts:226](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L226)

___

### decodeEventsSync

▸ **decodeEventsSync**(`events`): [`DecodedEvent`](../interfaces/DecodedEvent.md)[]

#### Parameters

| Name | Type |
| :------ | :------ |
| `events` | [`Event`](../interfaces/Event.md)[] |

#### Returns

[`DecodedEvent`](../interfaces/DecodedEvent.md)[]

#### Defined in

[index.d.ts:227](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L227)

___

### decodeLogs

▸ **decodeLogs**(`logs`): `Promise`\<[`DecodedEvent`](../interfaces/DecodedEvent.md)[]\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `logs` | [`Log`](../interfaces/Log.md)[] |

#### Returns

`Promise`\<[`DecodedEvent`](../interfaces/DecodedEvent.md)[]\>

#### Defined in

[index.d.ts:224](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L224)

___

### decodeLogsSync

▸ **decodeLogsSync**(`logs`): [`DecodedEvent`](../interfaces/DecodedEvent.md)[]

#### Parameters

| Name | Type |
| :------ | :------ |
| `logs` | [`Log`](../interfaces/Log.md)[] |

#### Returns

[`DecodedEvent`](../interfaces/DecodedEvent.md)[]

#### Defined in

[index.d.ts:225](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L225)

___

### new

▸ **new**(`jsonAbis`): [`Decoder`](Decoder.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `jsonAbis` | `Record`\<`string`, `any`\> |

#### Returns

[`Decoder`](Decoder.md)

#### Defined in

[index.d.ts:223](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L223)
