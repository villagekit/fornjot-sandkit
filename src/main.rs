use anyhow::Error;
use deno_core::resolve_path;
use fj_operations::shape_processor::ShapeProcessor;
use futures_lite::future;

mod model;
mod module;
mod viewer;

use crate::model::ModelLoader;
use crate::viewer::Viewer;

fn main() -> Result<(), Error> {
    let mut loader = ModelLoader::new();
    let current_dir = std::env::current_dir().expect("Unable to get CWD");
    let sketch_path = resolve_path("./sketches/demo.ts", &current_dir)?;

    future::block_on(loader.load(sketch_path))?;

    let shape = future::block_on(loader.get_shape())?;
    let shape_processor = ShapeProcessor { tolerance: None };
    let processed_shape = shape_processor.process(&shape)?;

    let mut viewer = Viewer::new(processed_shape)?;

    while viewer.render() {
        viewer.step()
    }

    Ok(())
}
