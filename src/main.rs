use deno_core::resolve_path;
use futures_lite::future;

mod engine;
mod loader;

use crate::engine::Engine;

fn main() {
    let shape_processor = ShapeProcessor {
        tolerance: args.tolerance,
    };
    let mut engine = Engine::new();

    let current_dir = std::env::current_dir().expect("Unable to get CWD");
    let sketch_path = resolve_path("./sketches/demo.ts", &current_dir).unwrap();

    future::block_on(engine.compile(sketch_path)).unwrap();

    future::block_on(model.engine.run(app.time)).unwrap();
}
