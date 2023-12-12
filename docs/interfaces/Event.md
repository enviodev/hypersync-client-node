[@envio-dev/hypersync-client](../README.md) / [Exports](../modules.md) / Event

# Interface: Event

Data relating to a single event (log)

## Table of contents

### Properties

- [block](Event.md#block)
- [log](Event.md#log)
- [transaction](Event.md#transaction)

## Properties

### block

• `Optional` **block**: [`Block`](Block.md)

Block that this event happened in

#### Defined in

[index.d.ts:104](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L104)

___

### log

• **log**: [`Log`](Log.md)

Evm log data

#### Defined in

[index.d.ts:106](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L106)

___

### transaction

• `Optional` **transaction**: [`Transaction`](Transaction.md)

Transaction that triggered this event

#### Defined in

[index.d.ts:102](https://github.com/Float-Capital/hypersync-client-node/blob/8a88f3d/index.d.ts#L102)
