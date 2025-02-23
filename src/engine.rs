use anyhow::Error;
use ash::{
    ext::debug_report::Instance,
    khr::surface,
    vk::{DebugUtilsMessengerEXT, Image, PhysicalDevice, Queue, SurfaceKHR, SwapchainKHR},
    Device, Entry,
};
use debugger::setup_debugger;
use instance::create_instance;
use queues::QueueIndices;
use swapchain::SwapchainSupportDetails;
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};
mod debugger;
mod device;
mod errors;
mod instance;
mod swapchain;
mod queues;
mod physical_devices;

pub struct Engine {
    entry: Entry,
    instance: ash::Instance,
    debug_instance: ash::ext::debug_utils::Instance,
    debug_messenger: DebugUtilsMessengerEXT,
    queue_indices: QueueIndices,
    physical_device: PhysicalDevice,
    surface_instance: ash::khr::surface::Instance,
    surface_khr: SurfaceKHR,
    device: Device,
    graphics_queue: Queue,
    presentation_queue: Queue,
    swapchain_device: ash::khr::swapchain::Device,
    swapchain: SwapchainKHR,
    images: Vec<Image>,
}

impl Engine {
    pub fn new(window: &Window) -> Result<Engine, Error> {
        let width = window.inner_size().width;
        let height = window.inner_size().height;
        let (entry, instance) = create_instance(&window).unwrap();
        let (debug_instance, debug_messenger) = setup_debugger(&entry, &instance);
        let surface_instance = ash::khr::surface::Instance::new(&entry, &instance);
        let surface_khr = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.display_handle().unwrap().as_raw(),
                window.window_handle().unwrap().as_raw(),
                None,
            )
            .unwrap()
        };
        let physical_device =
            physical_devices::find_physical_device(&instance, &surface_instance, surface_khr)
                .unwrap();
        let queue_indices = QueueIndices::find_queue_family_indices(
            physical_device,
            &instance,
            &surface_instance,
            &surface_khr,
        )
        .unwrap();
        let device = device::create_device(&instance, physical_device, queue_indices).unwrap();
        let graphics_queue =
            unsafe { device.get_device_queue(queue_indices.graphics_queue_index.unwrap(), 0) };
        let presentation_queue =
            unsafe { device.get_device_queue(queue_indices.presentation_queue_index.unwrap(), 0) };
        let swapchain_device = ash::khr::swapchain::Device::new(&instance, &device);
        let swapchain_support_details = SwapchainSupportDetails::query_swapchain_support(
            &surface_instance,
            physical_device,
            surface_khr,
        )?;
        let swapchain = swapchain::create_swapchain(
            &swapchain_device,
            swapchain_support_details,
            queue_indices,
            surface_khr,
            width,
            height,
        )
        .unwrap();
        let images = swapchain::create_swapchain_images(&swapchain_device, swapchain)?;

        Ok(Engine {
            entry,
            instance,
            debug_instance,
            debug_messenger,
            physical_device,
            queue_indices,
            surface_instance,
            surface_khr,
            device,
            graphics_queue,
            presentation_queue,
            swapchain_device,
            swapchain,
            images,
        })
    }
}
