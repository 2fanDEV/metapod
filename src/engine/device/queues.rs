use ash::{
    vk::{PhysicalDevice, QueueFlags, SurfaceKHR},
    Instance,
};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct QueueIndices {
    pub graphics_queue_index: Option<u32>,
    pub presentation_queue_index: Option<u32>,
}

#[derive(Error, Debug)]
pub enum QueueFamilyIndicesError {
    #[error("No families found")]
    NotFoundError,
}

impl QueueIndices {
    pub fn find_queue_family_indices(
        physical_device: PhysicalDevice,
        instance: &Instance,
        surface_instance: &ash::khr::surface::Instance,
        surface: &SurfaceKHR,
    ) -> Result<QueueIndices, QueueFamilyIndicesError> {
        let q_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let (queue_idx, queue_fam) = match q_family_properties
            .iter()
            .enumerate()
            .find(|&q_family| q_family.1.queue_flags.contains(QueueFlags::GRAPHICS))
        {
            Some(q_fam) => q_fam,
            None => return Err(QueueFamilyIndicesError::NotFoundError),
        };

        let surface_support = unsafe {
            surface_instance
                .get_physical_device_surface_support(physical_device, queue_idx as u32, *surface)
                .unwrap()
        };
        let presentation_queue_index = match surface_support {
            true => Some(queue_idx as u32),
            false => None,
        };

        Ok(QueueIndices {
            graphics_queue_index: Some(queue_idx as u32),
            presentation_queue_index,
        })
    }

    pub fn is_complete(self) -> bool {
        self.graphics_queue_index.is_some() && self.presentation_queue_index.is_some()
    }
}
