use winit::{application::ApplicationHandler, window::{Window, WindowAttributes}};

use crate::engine::{self, Engine};
#[derive(Default)]
pub struct App {
    window: Option<Window>,
    engine: Option<Engine>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
       let window_attributes = WindowAttributes::default();
       self.window = Some(event_loop.create_window(window_attributes).unwrap());
       engine::Engine::new();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        
    }
}
