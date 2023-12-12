[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / TransactionSelection

# Interface: TransactionSelection

## Table of contents

### Properties

- [from](TransactionSelection.md#from)
- [sighash](TransactionSelection.md#sighash)
- [status](TransactionSelection.md#status)
- [to](TransactionSelection.md#to)

## Properties

### from

• `Optional` **from**: `string`[]

Address the transaction should originate from. If transaction.from matches any of these, the transaction
 will be returned. Keep in mind that this has an and relationship with to filter, so each transaction should
 match both of them. Empty means match all.

#### Defined in

[index.d.ts:32](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L32)

___

### sighash

• `Optional` **sighash**: `string`[]

If first 4 bytes of transaction input matches any of these, transaction will be returned. Empty means match all.

#### Defined in

[index.d.ts:40](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L40)

___

### status

• `Optional` **status**: `number`

If tx.status matches this it will be returned.

#### Defined in

[index.d.ts:42](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L42)

___

### to

• `Optional` **to**: `string`[]

Address the transaction should go to. If transaction.to matches any of these, the transaction will
 be returned. Keep in mind that this has an and relationship with from filter, so each transaction should
 match both of them. Empty means match all.

#### Defined in

[index.d.ts:38](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L38)
