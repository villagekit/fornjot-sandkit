use nannou::prelude::*;
use tokio::runtime::{Builder as RuntimeBuilder, Runtime};

mod engine;

use crate::engine::Engine;

struct Model {
    engine: Engine,
    runtime: Runtime,
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    let engine = Engine::new();

    let runtime = RuntimeBuilder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    Model { engine, runtime }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model
        .runtime
        .block_on(
            model.engine.run(
                "
            console.log(time)
            "
                .to_string(),
                app.time,
            ),
        )
        .unwrap();
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
