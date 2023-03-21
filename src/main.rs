use nannou::prelude::*;

mod engine;

use crate::engine::Engine;

struct Model {
    engine: Engine,
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

    Model { engine }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model
        .engine
        .run(
            "
            console.log(time)
            ",
            app.time,
        )
        .unwrap();
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
