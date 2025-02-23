use ash::vk::{
    ColorSpaceKHR, CompositeAlphaFlagsKHR, Extent2D, Format, Image, ImageUsageFlags, PhysicalDevice, PresentModeKHR, SharingMode, SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR, SwapchainCreateFlagsKHR, SwapchainCreateInfoKHR, SwapchainKHR
};
use thiserror::Error;

use crate::engine::queues::QueueIndices;

pub struct SwapchainSupportDetails {
   pub surface_capabilities: SurfaceCapabilitiesKHR,
   pub  surface_formats: Vec<SurfaceFormatKHR>,
   pub present_modes: Vec<PresentModeKHR>,
}

#[derive(Error, Debug)]
pub enum SwapchainSupportError {
    #[error("Couldn't retrieve format")]
    FailedToGetSupportDetails,
}

#[derive(Error, Debug)]
pub enum SwapchainCreationError {
    #[error("Failed to create the swapchain,\n ERR_MSG: {err_message:?}")]
    SwapchainCreationError {
        err_message: ash::vk::Result
    }
}

impl SwapchainSupportDetails {
    pub fn query_swapchain_support(
        surface_instance: &ash::khr::surface::Instance,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
    ) -> Result<SwapchainSupportDetails, SwapchainSupportError> {
        Ok(SwapchainSupportDetails {
            surface_capabilities: unsafe {
                surface_instance
                    .get_physical_device_surface_capabilities(physical_device, surface)
                    .unwrap()
            },
            surface_formats: unsafe {
                surface_instance
                    .get_physical_device_surface_formats(physical_device, surface)
                    .unwrap()
            },
            present_modes: unsafe {
                surface_instance
                    .get_physical_device_surface_present_modes(physical_device, surface)
                    .unwrap()
            },
        })
    }
}

pub fn create_swapchain(
    swapchain_device: &ash::khr::swapchain::Device,
    swapchain_support_details: SwapchainSupportDetails,
    queue_indices: QueueIndices,
    surface: SurfaceKHR,
    width: u32,
    height: u32
) -> Result<SwapchainKHR, SwapchainCreationError> {
    let extent = Extent2D::default().height(height).width(width);
    
    let min_image_count = get_image_count(swapchain_support_details.surface_capabilities.min_image_count + 1, swapchain_support_details.surface_capabilities.max_image_count);

    let queue_family_indices = [queue_indices.graphics_queue_index.unwrap()];
    let create_info= SwapchainCreateInfoKHR::default()
        .flags(SwapchainCreateFlagsKHR::default())
        .image_usage(ImageUsageFlags::TRANSFER_DST | ImageUsageFlags::COLOR_ATTACHMENT)
        .image_extent(extent)
        .present_mode(PresentModeKHR::FIFO)
        .image_format(Format::B8G8R8A8_UNORM)
        .image_color_space(ColorSpaceKHR::SRGB_NONLINEAR)
        .image_sharing_mode(SharingMode::CONCURRENT)
        .image_array_layers(1)
        .queue_family_indices(&queue_family_indices)
        .pre_transform(swapchain_support_details.surface_capabilities.current_transform)
        .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
        .clipped(true)
        .surface(surface)
        .min_image_count(min_image_count);
    match unsafe { swapchain_device.create_swapchain(&create_info, None) } {
        Ok(swapchain) => Ok(swapchain),
        Err(err) => {
            Err(SwapchainCreationError::SwapchainCreationError { err_message: err})
        } ,
    }
}

fn get_image_count(min_image_count: u32, max_image_count: u32) -> u32 {
    u32::min(min_image_count, max_image_count)
}

pub fn create_swapchain_images(device: &ash::khr::swapchain::Device, swapchain: SwapchainKHR) -> Result<Vec<Image>, anyhow::Error> {
    unsafe { match device.get_swapchain_images(swapchain) {
        Ok(images) => Ok(images),
        Err(_) => {
            Err(anyhow::anyhow!("Failed to retrieve images"))
        },
    }
    }
}
