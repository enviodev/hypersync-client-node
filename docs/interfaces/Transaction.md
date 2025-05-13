[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / Transaction

# Interface: Transaction

Evm transaction object

See ethereum rpc spec for the meaning of fields

## Table of contents

### Properties

- [accessList](Transaction.md#accesslist)
- [authorizationList](Transaction.md#authorizationlist)
- [blobVersionedHashes](Transaction.md#blobversionedhashes)
- [blockHash](Transaction.md#blockhash)
- [blockNumber](Transaction.md#blocknumber)
- [chainId](Transaction.md#chainid)
- [contractAddress](Transaction.md#contractaddress)
- [cumulativeGasUsed](Transaction.md#cumulativegasused)
- [effectiveGasPrice](Transaction.md#effectivegasprice)
- [from](Transaction.md#from)
- [gas](Transaction.md#gas)
- [gasPrice](Transaction.md#gasprice)
- [gasUsed](Transaction.md#gasused)
- [gasUsedForL1](Transaction.md#gasusedforl1)
- [hash](Transaction.md#hash)
- [input](Transaction.md#input)
- [kind](Transaction.md#kind)
- [l1Fee](Transaction.md#l1fee)
- [l1FeeScalar](Transaction.md#l1feescalar)
- [l1GasPrice](Transaction.md#l1gasprice)
- [l1GasUsed](Transaction.md#l1gasused)
- [logsBloom](Transaction.md#logsbloom)
- [maxFeePerBlobGas](Transaction.md#maxfeeperblobgas)
- [maxFeePerGas](Transaction.md#maxfeepergas)
- [maxPriorityFeePerGas](Transaction.md#maxpriorityfeepergas)
- [nonce](Transaction.md#nonce)
- [r](Transaction.md#r)
- [root](Transaction.md#root)
- [s](Transaction.md#s)
- [status](Transaction.md#status)
- [to](Transaction.md#to)
- [transactionIndex](Transaction.md#transactionindex)
- [v](Transaction.md#v)
- [value](Transaction.md#value)
- [yParity](Transaction.md#yparity)

## Properties

### accessList

• `Optional` **accessList**: [`AccessList`](AccessList.md)[]

#### Defined in

[index.d.ts:371](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L371)

___

### authorizationList

• `Optional` **authorizationList**: [`Authorization`](Authorization.md)[]

#### Defined in

[index.d.ts:372](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L372)

___

### blobVersionedHashes

• `Optional` **blobVersionedHashes**: `string`[]

#### Defined in

[index.d.ts:374](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L374)

___

### blockHash

• `Optional` **blockHash**: `string`

#### Defined in

[index.d.ts:353](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L353)

___

### blockNumber

• `Optional` **blockNumber**: `number`

#### Defined in

[index.d.ts:354](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L354)

___

### chainId

• `Optional` **chainId**: `number`

#### Defined in

[index.d.ts:370](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L370)

___

### contractAddress

• `Optional` **contractAddress**: `string`

#### Defined in

[index.d.ts:378](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L378)

___

### cumulativeGasUsed

• `Optional` **cumulativeGasUsed**: `bigint`

#### Defined in

[index.d.ts:375](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L375)

___

### effectiveGasPrice

• `Optional` **effectiveGasPrice**: `bigint`

#### Defined in

[index.d.ts:376](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L376)

___

### from

• `Optional` **from**: `string`

#### Defined in

[index.d.ts:355](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L355)

___

### gas

• `Optional` **gas**: `bigint`

#### Defined in

[index.d.ts:356](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L356)

___

### gasPrice

• `Optional` **gasPrice**: `bigint`

#### Defined in

[index.d.ts:357](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L357)

___

### gasUsed

• `Optional` **gasUsed**: `bigint`

#### Defined in

[index.d.ts:377](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L377)

___

### gasUsedForL1

• `Optional` **gasUsedForL1**: `bigint`

#### Defined in

[index.d.ts:387](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L387)

___

### hash

• `Optional` **hash**: `string`

#### Defined in

[index.d.ts:358](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L358)

___

### input

• `Optional` **input**: `string`

#### Defined in

[index.d.ts:359](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L359)

___

### kind

• `Optional` **kind**: `number`

#### Defined in

[index.d.ts:380](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L380)

___

### l1Fee

• `Optional` **l1Fee**: `bigint`

#### Defined in

[index.d.ts:383](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L383)

___

### l1FeeScalar

• `Optional` **l1FeeScalar**: `number`

#### Defined in

[index.d.ts:386](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L386)

___

### l1GasPrice

• `Optional` **l1GasPrice**: `bigint`

#### Defined in

[index.d.ts:384](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L384)

___

### l1GasUsed

• `Optional` **l1GasUsed**: `bigint`

#### Defined in

[index.d.ts:385](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L385)

___

### logsBloom

• `Optional` **logsBloom**: `string`

#### Defined in

[index.d.ts:379](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L379)

___

### maxFeePerBlobGas

• `Optional` **maxFeePerBlobGas**: `bigint`

#### Defined in

[index.d.ts:373](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L373)

___

### maxFeePerGas

• `Optional` **maxFeePerGas**: `bigint`

#### Defined in

[index.d.ts:369](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L369)

___

### maxPriorityFeePerGas

• `Optional` **maxPriorityFeePerGas**: `bigint`

#### Defined in

[index.d.ts:368](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L368)

___

### nonce

• `Optional` **nonce**: `bigint`

#### Defined in

[index.d.ts:360](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L360)

___

### r

• `Optional` **r**: `string`

#### Defined in

[index.d.ts:365](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L365)

___

### root

• `Optional` **root**: `string`

#### Defined in

[index.d.ts:381](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L381)

___

### s

• `Optional` **s**: `string`

#### Defined in

[index.d.ts:366](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L366)

___

### status

• `Optional` **status**: `number`

#### Defined in

[index.d.ts:382](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L382)

___

### to

• `Optional` **to**: `string`

#### Defined in

[index.d.ts:361](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L361)

___

### transactionIndex

• `Optional` **transactionIndex**: `number`

#### Defined in

[index.d.ts:362](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L362)

___

### v

• `Optional` **v**: `string`

#### Defined in

[index.d.ts:364](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L364)

___

### value

• `Optional` **value**: `bigint`

#### Defined in

[index.d.ts:363](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L363)

___

### yParity

• `Optional` **yParity**: `string`

#### Defined in

[index.d.ts:367](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L367)
