**neon-embeddings** • Readme \| [API](globals.md)

***

# neon-embeddings

Use [neon](https://docs.rs/neon/latest/neon/) to wrap [fastembed-rs](https://docs.rs/fastembed/latest/fastembed/)
for generating vector embeddings with state of the art embedding models.

## Installing

Installing cpu-count requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

You can install the project with npm. In the project directory, run:

```sh
$ npm install
```

This fully installs the project, including installing any dependencies and running the build.

## Building

If you have already installed the project and only want to run the build, run:

```sh
$ npm run build
```

This command uses the [cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact) utility to run the Rust build and copy the built library into `./embeddings.node`.

## Exploring

After building cpu-count, you can explore its exports at the Node REPL:

```sh
$ npm install
$ npx ts-node
> import { EmbeddingsModel } from '.';
undefined
> const model = await EmbeddingsModel.init()
model_optimized.onnx [00:00:11] [██████████████████████████████████████████████████████████] 63.39 MiB/63.39 MiB 5.54 MiB/s (0s)
tokenizer.json [00:00:00] [██████████████████████████████████████████████████████] 694.72 KiB/694.72 KiB 1.49 MiB/s (0s)
config.json [00:00:00] [████████████████████████████████████████████████████████████████████████████████] 706 B/706 B 6.37 KiB/s (0s)
special_tokens_map.json [00:00:00] [█████████████████████████████████████████████████] 695 B/695 B 6.43 KiB/s (0s)
tokenizer_config.json [00:00:00] [█████████████████████████████████████████████████████████] 1.21 KiB/1.21 KiB 11.23 KiB/s (0s)undefined
> const embeddings = await model.embed(["hello, world"], 1)
undefined
> embeddings[0].length
384
> 
```

## API Docs

[EmbeddingsModel.md](./docs/classes/EmbeddingsModel.md)

## Scripts

In the project directory, you can run:

### `npm install`

Installs the project, including running `npm run build`.

### `npm build`

Builds the Node addon (`lib/embeddings.node`) from source.

Additional [`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html) arguments may be passed to `npm build` and `npm build-*` commands. For example, to enable a [cargo feature](https://doc.rust-lang.org/cargo/reference/features.html):

```
npm run build -- --feature=beetle
```

#### `npm build-debug`

Alias for `npm build`.

#### `npm build-release`

Same as [`npm build`](#npm-build) but, builds the module with the [`release`](https://doc.rust-lang.org/cargo/reference/profiles.html#release) profile. Release builds will compile slower, but run faster.

### `npm test`

Runs the unit tests by calling `cargo test` and `node --test`.
