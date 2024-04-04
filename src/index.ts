const { init, embed, embedQuery } = require('../lib/embeddings.node');

interface Model {
  embed(documents: string[], batchSize: number): Promise<number[][]>;
  embedQuery(query: string): Promise<number[][]>;
}

export class EmbeddingsModel implements Model {
  static async init(): Promise<EmbeddingsModel> {
    const model = await init();
    return new EmbeddingsModel(model);
  }

  constructor(private model: Model) {}
  
  public async embed(documents: string[], batchSize: number = 1): Promise<number[][]> {
    const embeddings = await embed.call(this.model, documents, batchSize);
    return embeddings;
  }

  public async embedQuery(query: string): Promise<number[][]> {
    const embeddings = await embedQuery.call(this.model, query);
    return embeddings;
  }
}


export default EmbeddingsModel.init;