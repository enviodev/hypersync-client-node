[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / BlockSelection

# Interface: BlockSelection

## Table of contents

### Properties

- [hash](BlockSelection.md#hash)
- [miner](BlockSelection.md#miner)

## Properties

### hash

• `Optional` **hash**: `string`[]

Hash of a block, any blocks that have one of these hashes will be returned.
Empty means match all.

#### Defined in

[index.d.ts:239](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L239)

___

### miner

• `Optional` **miner**: `string`[]

Miner address of a block, any blocks that have one of these miners will be returned.
Empty means match all.

#### Defined in

[index.d.ts:244](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L244)
