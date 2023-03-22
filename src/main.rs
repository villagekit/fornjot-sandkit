use deno_core::resolve_path;
use futures_lite::future;
use nannou::draw::Draw;
use nannou::prelude::*;

mod engine;
mod loader;

use crate::engine::Engine;

struct Model {
    engine: Engine,
    draw: Draw,
    fps: Option<f32>,
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
    let mut engine = Engine::new(draw.clone());

    let current_dir = std::env::current_dir().expect("Unable to get CWD");
    let sketch_path = resolve_path("./sketches/demo.ts", &current_dir).unwrap();

    future::block_on(engine.compile(sketch_path)).unwrap();

    Model {
        engine,
        draw: draw.clone(),
        fps: None,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let draw = &model.draw;

    draw.reset();
    draw.background().color(BLUE);

    future::block_on(model.engine.run(app.time)).unwrap();

    /*
    let n = 5000;
    let t = app.time * 0.1;
    for i in 0..n {
        let a = i as f32 / n as f32;
        let b = (a + t) % 1.0;
        let x = (b * PI * 16.0).sin() * 500.0 * a;
        let y = (b * PI * 16.0).cos() * 500.0 * a;
        draw.rect().x_y(x, y).w_h(4.0, 4.0).color(PLUM);
    }
    */

    match model.fps {
        Some(fps) => model.fps = Some((app.fps() + fps) / 2.0),
        None => model.fps = Some(app.fps()),
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = &model.draw;

    let fps = model.fps.unwrap_or(0.0).round();

    let win = app.window_rect();
    let win_p = win.pad(25.0);

    let rect = Rect::from_w_h(0.0, 0.0).top_left_of(win_p);
    draw.text(&fps.to_string()).xy(rect.xy()).font_size(20);

    draw.to_frame(app, &frame).unwrap();
}
