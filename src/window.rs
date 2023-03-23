// https://github.com/hannobraun/Fornjot/blob/main/crates/fj-window/src/window.rs

use anyhow::Error;
use fj_viewer::{Screen, ScreenSize};
use winit::{
    event_loop::EventLoop,
    window::{WindowBuilder, WindowId},
};

/// Window abstraction providing details such as the width or height and easing initialization.
pub struct Window(winit::window::Window);

impl Window {
    /// Returns a new window with the given `EventLoop`.
    pub fn new<T>(event_loop: &EventLoop<T>) -> Result<Self, Error> {
        let window = WindowBuilder::new()
            .with_title("Mycelia")
            .with_maximized(true)
            .with_decorations(true)
            .with_transparent(false)
            .build(event_loop)?;

        Ok(Self(window))
    }

    pub fn id(&self) -> WindowId {
        self.0.id()
    }
}

impl Screen for Window {
    type Window = winit::window::Window;

    fn size(&self) -> ScreenSize {
        let size = self.0.inner_size();

        ScreenSize {
            width: size.width,
            height: size.height,
        }
    }

    fn window(&self) -> &winit::window::Window {
        &self.0
    }
}
