[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / LogSelection

# Interface: LogSelection

## Table of contents

### Properties

- [address](LogSelection.md#address)
- [topics](LogSelection.md#topics)

## Properties

### address

• `Optional` **address**: `string`[]

Address of the contract, any logs that has any of these addresses will be returned.
Empty means match all.

#### Defined in

[index.d.ts:19](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L19)

___

### topics

• `Optional` **topics**: `string`[][]

Topics to match, each member of the top level array is another array, if the nth topic matches any
 topic specified in topics[n] the log will be returned. Empty means match all.

#### Defined in

[index.d.ts:24](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L24)
