use anyhow::Error;
use deno_core::resolve_path;
use egui::RawInput;
use fj_operations::shape_processor::ShapeProcessor;
use fj_viewer::{GuiState, StatusReport, Viewer};
use futures_lite::future;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod engine;
mod loader;
mod window;

use crate::engine::Engine;
use crate::window::Window;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;
    let mut viewer = future::block_on(Viewer::new(&window))?;

    let mut engine = Engine::new();
    let current_dir = std::env::current_dir().expect("Unable to get CWD");
    let sketch_path = resolve_path("./sketches/demo.ts", &current_dir)?;

    future::block_on(engine.load(sketch_path))?;

    let shape = future::block_on(engine.shape())?;
    let shape_processor = ShapeProcessor { tolerance: None };
    let processed_shape = shape_processor.process(&shape)?;

    viewer.handle_shape_update(processed_shape);

    let pixels_per_point = 1_f32;
    let egui_input = RawInput {
        ..Default::default()
    };
    let gui_state = GuiState {
        status: &StatusReport::new(),
        model_available: true,
    };
    viewer.draw(pixels_per_point, egui_input, gui_state);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
