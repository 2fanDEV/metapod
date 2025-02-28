use ash::{
    Device, Instance,
};
use ash::vk::{
        DeviceCreateFlags, DeviceCreateInfo, DeviceQueueCreateFlags, DeviceQueueCreateInfo,
        PhysicalDevice, SurfaceKHR, KHR_SWAPCHAIN_NAME,
    };
use super::queues::QueueIndices;

use super::errors::device_error::DeviceError;


pub fn create_device(
    instance: &Instance,
    physical_device: PhysicalDevice,
    queue_indices: QueueIndices,
) -> Result<Device, DeviceError> {
    let features = unsafe { instance.get_physical_device_features(physical_device) };
    let device_extensions = vec![KHR_SWAPCHAIN_NAME.as_ptr()];
    let device_queue_create_info = &[DeviceQueueCreateInfo::default()
        .queue_family_index(queue_indices.graphics_queue_index.unwrap())
        .queue_priorities(&[1.0])
        .flags(DeviceQueueCreateFlags::default())]  ;

    let create_info = DeviceCreateInfo::default()
        .queue_create_infos(device_queue_create_info)
        .enabled_features(&features)
        .enabled_extension_names(&device_extensions)
        .flags(DeviceCreateFlags::empty());

    let device = unsafe {
        instance
            .create_device(physical_device, &create_info, None)
            .unwrap()
    };
    Ok(device)
}
