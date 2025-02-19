use app::App;
use log::LevelFilter;
use winit::event_loop::{ActiveEventLoop, EventLoop};

mod app;
mod engine;

fn main() {
    let mut app = App::default();
    let event_loop = EventLoop::new().unwrap();
    let _ = env_logger::builder().filter_level(LevelFilter::Debug).try_init();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
    println!("Hello, world!");
}
