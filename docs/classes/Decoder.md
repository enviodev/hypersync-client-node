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
- [disableChecksummedAddresses](Decoder.md#disablechecksummedaddresses)
- [enableChecksummedAddresses](Decoder.md#enablechecksummedaddresses)
- [fromSignatures](Decoder.md#fromsignatures)
- [fromSignaturesWithChecksum](Decoder.md#fromsignatureswithchecksum)

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

[index.d.ts:574](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L574)

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

[index.d.ts:575](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L575)

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

[index.d.ts:572](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L572)

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

[index.d.ts:573](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L573)

___

### disableChecksummedAddresses

▸ **disableChecksummedAddresses**(): `void`

#### Returns

`void`

#### Defined in

[index.d.ts:571](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L571)

___

### enableChecksummedAddresses

▸ **enableChecksummedAddresses**(): `void`

#### Returns

`void`

#### Defined in

[index.d.ts:570](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L570)

___

### fromSignatures

▸ **fromSignatures**(`signatures`): [`Decoder`](Decoder.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `signatures` | `string`[] |

#### Returns

[`Decoder`](Decoder.md)

#### Defined in

[index.d.ts:568](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L568)

___

### fromSignaturesWithChecksum

▸ **fromSignaturesWithChecksum**(`signatures`, `checksum`): [`Decoder`](Decoder.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `signatures` | `string`[] |
| `checksum` | `boolean` |

#### Returns

[`Decoder`](Decoder.md)

#### Defined in

[index.d.ts:569](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L569)
