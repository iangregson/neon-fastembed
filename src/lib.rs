use neon::prelude::*;

use fastembed::TextEmbedding;
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

struct Model(TextEmbedding);

// Return a global tokio runtime or create one if it doesn't exist.
// Throws a JavaScript exception if the `Runtime` fails to create.
// https://github.com/neon-bindings/examples/blob/main/examples/tokio-fetch/src/lib.rs
fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();

    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

fn embedding_vec_to_array<'a, C: Context<'a>>(
    vec: Vec<Vec<f32>>,
    cx: &mut C,
) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len());

    for (i, embedding) in vec.iter().enumerate() {
        let b = JsArray::new(cx, embedding.len());

        for (j, n) in embedding.iter().enumerate() {
            let v = cx.number(*n);
            b.set(cx, j as u32, v)?;
        }
        a.set(cx, i as u32, b)?;
    }

    Ok(a)
}

// Clean-up when model is garbage collected, could go here
// but, it's not needed,
impl Finalize for Model {}

fn init(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    // This task will _not_ block the JavaScript main thread.
    rt.spawn(async move {
        deferred.settle_with(&channel, move |mut cx| {
            let model = TextEmbedding::try_new(Default::default())
                .or_else(|err| cx.throw_error(err.to_string()))?;

            Ok(cx.boxed(Model(model)))
        });
    });

    Ok(promise)
}

fn embed(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let rt = runtime(&mut cx)?;
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // Get the `this` value as a `JsBox<Model>`
    let model = cx.this::<JsBox<Model>>()?;

    let batch_size: Handle<JsNumber> = cx.argument(1).or_else(|_| Ok(cx.number(1)))?;
    let batch_size: usize = batch_size.value(&mut cx) as usize;

    let inputs: Handle<JsArray> = cx.argument(0)?;
    let inputs: Vec<Handle<JsValue>> = inputs.to_vec(&mut cx)?;
    let mut documents: Vec<String> = Vec::new();
    for item in inputs.iter() {
        let js_string: Handle<JsString> = item.to_string(&mut cx)?;
        documents.push("passage: ".to_owned() + &js_string.value(&mut cx));
    }
    let embeddings = model
        .0
        .embed(documents, Some(batch_size))
        .or_else(|err| cx.throw_error(err.to_string()))?;

    // This task will _not_ block the JavaScript main thread.
    rt.spawn(async move {
        deferred.settle_with(&channel, move |mut cx| {
            let ret = embedding_vec_to_array(embeddings, &mut cx)?;
            Ok(ret)
        });
    });

    Ok(promise)
}

fn embed_query(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let rt = runtime(&mut cx)?;
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // Get the `this` value as a `JsBox<Model>`
    let model = cx.this::<JsBox<Model>>()?;

    let input: Handle<JsString> = cx.argument(0)?;
    let documents = vec!["query: ".to_owned() + &input.value(&mut cx)];
    let embeddings = model
        .0
        .embed(documents, Some(1))
        .or_else(|err| cx.throw_error(err.to_string()))?;

    // This task will _not_ block the JavaScript main thread.
    rt.spawn(async move {
        deferred.settle_with(&channel, move |mut cx| {
            let ret = embedding_vec_to_array(embeddings, &mut cx)?;
            Ok(ret)
        });
    });

    Ok(promise)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("init", init)?;
    cx.export_function("embed", embed)?;
    cx.export_function("embedQuery", embed_query)?;
    Ok(())
}
