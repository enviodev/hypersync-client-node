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

[index.d.ts:79](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L79)

___

### topics

• `Optional` **topics**: `string`[][]

Topics to match, each member of the top level array is another array, if the nth topic matches any
 topic specified in topics[n] the log will be returned. Empty means match all.

#### Defined in

[index.d.ts:84](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L84)
