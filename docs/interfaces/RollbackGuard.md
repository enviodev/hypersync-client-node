[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / RollbackGuard

# Interface: RollbackGuard

## Table of contents

### Properties

- [blockNumber](RollbackGuard.md#blocknumber)
- [firstBlockNumber](RollbackGuard.md#firstblocknumber)
- [firstParentHash](RollbackGuard.md#firstparenthash)
- [hash](RollbackGuard.md#hash)
- [timestamp](RollbackGuard.md#timestamp)

## Properties

### blockNumber

• **blockNumber**: `number`

Block number of the last scanned block

#### Defined in

[index.d.ts:495](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L495)

___

### firstBlockNumber

• **firstBlockNumber**: `number`

Block number of the first scanned block in memory.

This might not be the first scanned block. It only includes blocks that are in memory (possible to be rolled back).

#### Defined in

[index.d.ts:505](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L505)

___

### firstParentHash

• **firstParentHash**: `string`

Parent hash of the first scanned block in memory.

This might not be the first scanned block. It only includes blocks that are in memory (possible to be rolled back).

#### Defined in

[index.d.ts:511](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L511)

___

### hash

• **hash**: `string`

Block hash of the last scanned block

#### Defined in

[index.d.ts:499](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L499)

___

### timestamp

• **timestamp**: `number`

Block timestamp of the last scanned block

#### Defined in

[index.d.ts:497](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L497)
