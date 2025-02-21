use crate::engine::device::queues::QueueIndices;
use ash::{
    vk::{PhysicalDevice, PhysicalDeviceProperties, PhysicalDeviceType, SurfaceKHR},
    Instance,
};
use log::{debug, info};

use crate::engine::errors::device_error::DeviceError;

pub fn find_physical_device(
    instance: &Instance,
    surface_instance: &ash::khr::surface::Instance,
    surface: SurfaceKHR,
) -> Result<PhysicalDevice, DeviceError> {
    let enumerated_devices = unsafe { instance.enumerate_physical_devices() };
    debug!("{:?}", enumerated_devices);
    let physical_devices_vec: Vec<PhysicalDevice> = match enumerated_devices {
        Ok(physical_devices) => physical_devices,
        Err(err) => {
            return Err(DeviceError::NoPhysicalDeviceFound);
        }
    };

    let physical_devices = physical_devices_vec
        .iter()
        .filter(|&device| is_device_suitable(*device, instance, &surface_instance, surface))
        .collect::<Vec<&PhysicalDevice>>();
    info!("Found {} physical devices", physical_devices.len());
    Ok(**physical_devices.first().unwrap())
}

fn is_device_suitable(
    physical_device: PhysicalDevice,
    instance: &Instance,
    surface_instance: &ash::khr::surface::Instance,
    surface: SurfaceKHR,
) -> bool {
    let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
    let device_features = unsafe { instance.get_physical_device_features(physical_device) };
    let queue_family_indices = match QueueIndices::find_queue_family_indices(
        physical_device,
        instance,
        &surface_instance,
        &surface,
    ) {
        Ok(q_family) => q_family,
        Err(err) => panic!("{}", err),
    };
    queue_family_indices.is_complete()
}
