[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / TransactionSelection

# Interface: TransactionSelection

## Table of contents

### Properties

- [authorizationList](TransactionSelection.md#authorizationlist)
- [contractAddress](TransactionSelection.md#contractaddress)
- [from](TransactionSelection.md#from)
- [kind](TransactionSelection.md#kind)
- [sighash](TransactionSelection.md#sighash)
- [status](TransactionSelection.md#status)
- [to](TransactionSelection.md#to)

## Properties

### authorizationList

• `Optional` **authorizationList**: [`AuthorizationSelection`](AuthorizationSelection.md)[]

If transaction.authorization_list matches any of these values, the transaction will be returned.

#### Defined in

[index.d.ts:107](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L107)

___

### contractAddress

• `Optional` **contractAddress**: `string`[]

#### Defined in

[index.d.ts:105](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L105)

___

### from

• `Optional` **from**: `string`[]

Address the transaction should originate from. If transaction.from matches any of these, the transaction
 will be returned. Keep in mind that this has an and relationship with to filter, so each transaction should
 match both of them. Empty means match all.

#### Defined in

[index.d.ts:92](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L92)

___

### kind

• `Optional` **kind**: `number`[]

If transaction.type matches any of these values, the transaction will be returned

#### Defined in

[index.d.ts:104](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L104)

___

### sighash

• `Optional` **sighash**: `string`[]

If first 4 bytes of transaction input matches any of these, transaction will be returned. Empty means match all.

#### Defined in

[index.d.ts:100](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L100)

___

### status

• `Optional` **status**: `number`

If tx.status matches this it will be returned.

#### Defined in

[index.d.ts:102](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L102)

___

### to

• `Optional` **to**: `string`[]

Address the transaction should go to. If transaction.to matches any of these, the transaction will
 be returned. Keep in mind that this has an and relationship with from filter, so each transaction should
 match both of them. Empty means match all.

#### Defined in

[index.d.ts:98](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L98)
