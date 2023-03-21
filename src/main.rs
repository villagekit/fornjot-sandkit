use nannou::draw::Draw;
use nannou::prelude::*;
use tokio::runtime::{Builder as RuntimeBuilder, Runtime};

mod engine;

use crate::engine::Engine;

struct Model {
    engine: Engine,
    runtime: Runtime,
    draw: Draw,
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(app: &App) -> Model {
    let draw = app.draw();
    let engine = Engine::new(draw.clone());

    let runtime = RuntimeBuilder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    Model {
        engine,
        runtime,
        draw: draw.clone(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.draw.background().color(BLACK);

    model
        .runtime
        .block_on(
            model.engine.run(
                "
const n = 5000
const t = time * 0.1
for (let i = 0; i < n; i++) {
    const x = Math.sin(n + t)
    const y = Math.cos(n + t)
    rect(x, y)
}
"
                .to_string(),
                app.time,
            ),
        )
        .unwrap();
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    model.draw.to_frame(app, &frame).unwrap();
}
