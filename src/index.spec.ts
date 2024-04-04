import { assert } from "chai";
import { describe, test, before } from "node:test";

import { EmbeddingsModel } from '.';

describe('tests', async () => {
  const DEFAULT_BATCH_SIZE = 256;
  const EMBEDDING_OUT_DIMENSIONS = 384;

  let model: EmbeddingsModel;

  before(async () => {
    model = await EmbeddingsModel.init();
  });

  test('model is ok', async () => {
    assert.isOk(model);
  });

  test('generates passage embeddings', async () => {
    const documents = [
      'hello, world',
      'neon makes node rusty',
      'to fastembed all the things',
    ];
    const embeddings = await model.embed(documents, DEFAULT_BATCH_SIZE);

    assert.lengthOf(embeddings, documents.length);
    
    for (const embedding of embeddings) {
      assert.lengthOf(embedding, EMBEDDING_OUT_DIMENSIONS);
    }
  });

  test('generates query embeddings', async () => {
    const query = 'hello, world';
    const embeddings = await model.embedQuery(query);

    assert.lengthOf(embeddings, 1);
    assert.lengthOf(embeddings[0], EMBEDDING_OUT_DIMENSIONS);
  });
}); 
