use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("There is no suitable PhysicalDevice on this device")]
    NoPhysicalDeviceFound
}
