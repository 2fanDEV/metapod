use ash::Instance;
mod instance;
mod errors;

pub struct Engine {
    instance: Instance,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
        instance: instance::create_instance().unwrap()
       }
    }
}

