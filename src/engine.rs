use ash::Instance;
use winit::window::Window;
mod instance;
mod errors;

pub struct Engine {
    instance: Instance,
}

impl Engine {
    pub fn new(window: Window) -> Engine {
        Engine {
        instance: instance::create_instance(&window).unwrap()
       }
    }
}

