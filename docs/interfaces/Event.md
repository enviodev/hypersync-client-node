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

[index.d.ts:327](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L327)

___

### log

• **log**: [`Log`](Log.md)

Evm log data

#### Defined in

[index.d.ts:329](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L329)

___

### transaction

• `Optional` **transaction**: [`Transaction`](Transaction.md)

Transaction that triggered this event

#### Defined in

[index.d.ts:325](https://github.com/Float-Capital/hypersync-client-node/blob/4ee0d9475a267b3a97cbbd6004114b9ba5d98295/index.d.ts#L325)
