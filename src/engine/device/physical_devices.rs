use std::collections::{BTreeSet, HashSet};

use crate::engine::{device::queues::QueueIndices, swapchain::SwapchainSupportDetails};
use ash::{
    vk::{ExtensionProperties, PhysicalDevice, SurfaceKHR},
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

fn check_device_extensions(
    instance: &Instance,
    physical_device: PhysicalDevice,
    device_extension_properties: Vec<&str>,
) -> bool {
    let mut count = 0;
    let available_extension_properties = unsafe {
        instance
            .enumerate_device_extension_properties(physical_device)
            .unwrap()
            .iter()
            .map(|extension| {
                extension
                    .extension_name_as_c_str()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
            })
            .collect::<Vec<_>>()
    };

    for extension in device_extension_properties.clone() {
        if available_extension_properties.contains(&extension.to_owned()) {
            count = count + 1;
        }
    }
    count == device_extension_properties.len()
}

fn is_device_suitable(
    physical_device: PhysicalDevice,
    instance: &Instance,
    surface_instance: &ash::khr::surface::Instance,
    surface: SurfaceKHR,
) -> bool {
    let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
    let device_features = unsafe { instance.get_physical_device_features(physical_device) };
    let extensions = vec!["VK_KHR_SWAPCHAIN_EXTENSION_NAME"];
    let queue_family_indices = match QueueIndices::find_queue_family_indices(
        physical_device,
        instance,
        &surface_instance,
        &surface,
    ) {
        Ok(q_family) => q_family,
        Err(err) => panic!("{}", err),
    };

    let extensions_supported = check_device_extensions(instance, physical_device, extensions);
    let swapchain_support = match SwapchainSupportDetails::query_swapchain_support(
        surface_instance,
        physical_device,
        surface,
    ) {
        Ok(support_details) => support_details,
        Err(err) => panic!("{:?}", err),
    };
    debug!("{}", extensions_supported);
    queue_family_indices.is_complete()
        && extensions_supported
        && !swapchain_support.surface_formats.is_empty()
        && !swapchain_support.present_modes.is_empty()
}
