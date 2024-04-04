**neon-embeddings** • [Readme](../README.md) \| [API](../globals.md)

***

[neon-embeddings](../README.md) / EmbeddingsModel

# Class: EmbeddingsModel

## Implements

- `Model`

## Constructors

### new EmbeddingsModel(model)

> **new EmbeddingsModel**(`model`): [`EmbeddingsModel`](EmbeddingsModel.md)

#### Parameters

• **model**: `Model`

#### Returns

[`EmbeddingsModel`](EmbeddingsModel.md)

#### Source

index.ts:14

## Properties

### model

> **`private`** **model**: `Model`

#### Source

index.ts:14

## Methods

### embed()

> **embed**(`documents`, `batchSize`): `Promise`\<`number`[][]\>

#### Parameters

• **documents**: `string`[]

• **batchSize**: `number`= `1`

#### Returns

`Promise`\<`number`[][]\>

#### Implementation of

`Model.embed`

#### Source

index.ts:16

***

### embedQuery()

> **embedQuery**(`query`): `Promise`\<`number`[][]\>

#### Parameters

• **query**: `string`

#### Returns

`Promise`\<`number`[][]\>

#### Implementation of

`Model.embedQuery`

#### Source

index.ts:21

***

### init()

> **`static`** **init**(): `Promise`\<[`EmbeddingsModel`](EmbeddingsModel.md)\>

#### Returns

`Promise`\<[`EmbeddingsModel`](EmbeddingsModel.md)\>

#### Source

index.ts:9
