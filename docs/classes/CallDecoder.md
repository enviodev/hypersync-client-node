[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / CallDecoder

# Class: CallDecoder

## Table of contents

### Constructors

- [constructor](CallDecoder.md#constructor)

### Methods

- [decodeImpl](CallDecoder.md#decodeimpl)
- [decodeInputs](CallDecoder.md#decodeinputs)
- [decodeInputsSync](CallDecoder.md#decodeinputssync)
- [decodeTracesInput](CallDecoder.md#decodetracesinput)
- [decodeTracesInputSync](CallDecoder.md#decodetracesinputsync)
- [decodeTransactionsInput](CallDecoder.md#decodetransactionsinput)
- [decodeTransactionsInputSync](CallDecoder.md#decodetransactionsinputsync)
- [fromSignatures](CallDecoder.md#fromsignatures)
- [fromSignaturesWithChecksum](CallDecoder.md#fromsignatureswithchecksum)

## Constructors

### constructor

• **new CallDecoder**(): [`CallDecoder`](CallDecoder.md)

#### Returns

[`CallDecoder`](CallDecoder.md)

## Methods

### decodeImpl

▸ **decodeImpl**(`input`): [`DecodedSolValue`](../interfaces/DecodedSolValue.md)[]

#### Parameters

| Name | Type |
| :------ | :------ |
| `input` | `string` |

#### Returns

[`DecodedSolValue`](../interfaces/DecodedSolValue.md)[]

#### Defined in

[index.d.ts:586](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L586)

___

### decodeInputs

▸ **decodeInputs**(`inputs`): `Promise`\<[`DecodedSolValue`](../interfaces/DecodedSolValue.md)[][]\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `inputs` | `string`[] |

#### Returns

`Promise`\<[`DecodedSolValue`](../interfaces/DecodedSolValue.md)[][]\>

#### Defined in

[index.d.ts:580](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L580)

___

### decodeInputsSync

▸ **decodeInputsSync**(`inputs`): [`DecodedSolValue`](../interfaces/DecodedSolValue.md)[][]

#### Parameters

| Name | Type |
| :------ | :------ |
| `inputs` | `string`[] |

#### Returns

[`DecodedSolValue`](../interfaces/DecodedSolValue.md)[][]

#### Defined in

[index.d.ts:583](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L583)

___

### decodeTracesInput

▸ **decodeTracesInput**(`traces`): `Promise`\<[`DecodedSolValue`](../interfaces/DecodedSolValue.md)[][]\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `traces` | [`Trace`](../interfaces/Trace.md)[] |

#### Returns

`Promise`\<[`DecodedSolValue`](../interfaces/DecodedSolValue.md)[][]\>

#### Defined in

[index.d.ts:582](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L582)

___

### decodeTracesInputSync

▸ **decodeTracesInputSync**(`traces`): [`DecodedSolValue`](../interfaces/DecodedSolValue.md)[][]

#### Parameters

| Name | Type |
| :------ | :------ |
| `traces` | [`Trace`](../interfaces/Trace.md)[] |

#### Returns

[`DecodedSolValue`](../interfaces/DecodedSolValue.md)[][]

#### Defined in

[index.d.ts:585](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L585)

___

### decodeTransactionsInput

▸ **decodeTransactionsInput**(`txs`): `Promise`\<[`DecodedSolValue`](../interfaces/DecodedSolValue.md)[][]\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `txs` | [`Transaction`](../interfaces/Transaction.md)[] |

#### Returns

`Promise`\<[`DecodedSolValue`](../interfaces/DecodedSolValue.md)[][]\>

#### Defined in

[index.d.ts:581](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L581)

___

### decodeTransactionsInputSync

▸ **decodeTransactionsInputSync**(`txs`): [`DecodedSolValue`](../interfaces/DecodedSolValue.md)[][]

#### Parameters

| Name | Type |
| :------ | :------ |
| `txs` | [`Transaction`](../interfaces/Transaction.md)[] |

#### Returns

[`DecodedSolValue`](../interfaces/DecodedSolValue.md)[][]

#### Defined in

[index.d.ts:584](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L584)

___

### fromSignatures

▸ **fromSignatures**(`signatures`): [`CallDecoder`](CallDecoder.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `signatures` | `string`[] |

#### Returns

[`CallDecoder`](CallDecoder.md)

#### Defined in

[index.d.ts:578](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L578)

___

### fromSignaturesWithChecksum

▸ **fromSignaturesWithChecksum**(`signatures`, `checksum`): [`CallDecoder`](CallDecoder.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `signatures` | `string`[] |
| `checksum` | `boolean` |

#### Returns

[`CallDecoder`](CallDecoder.md)

#### Defined in

[index.d.ts:579](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L579)
