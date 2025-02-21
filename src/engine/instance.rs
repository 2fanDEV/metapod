use std::ffi::CStr;

use anyhow::{anyhow, Error, Ok};
use ash::{
    vk::{ApplicationInfo, InstanceCreateInfo, KHR_PORTABILITY_ENUMERATION_NAME},
    Entry, Instance,
};
use winit::{raw_window_handle::HasDisplayHandle, window::Window};

use super::errors::instance_errors::EntryLoadingError;

static VULKAN_LIBRARY_LOCATION: &str = "/Users/tufan/VulkanSDK/1.3.296.0/macOS/lib/libvulkan.dylib";
static ENGINE_NAME: &CStr = c"Metapod";
static APP_NAME: &CStr = c"METAPOD";

pub fn create_instance(window: &Window) -> Result<Instance, Error> {

    let entry =  unsafe {
        match Entry::load_from(VULKAN_LIBRARY_LOCATION) {
            Ok(entry) => entry,
            Err(err) => {
                panic!(
                    "{}",
                    EntryLoadingError::InvalidLocation {
                        path: VULKAN_LIBRARY_LOCATION.to_string(),
                        msg: err.to_string()
                    }
                )
            }
        }
    };

    
    let enabled_layer_names = unsafe { entry.enumerate_instance_layer_properties()
    .iter().map(|layer| layer.layer_name.as_ptr()).collect::<Vec<*const i8>>()};

    let application_info = ApplicationInfo::default()
        .engine_name(ENGINE_NAME)
        .engine_version(1)
        .api_version(1)
        .application_name(APP_NAME)
        .application_version(1);

    let enabled_extension_names = get_enabled_extensions(window)?;
    let instance_create_info= InstanceCreateInfo::default()
        .application_info(&application_info)
        .enabled_extension_names(&enabled_extension_names)
        .enabled_layer_names(&enabled_layer_names);

    Ok(entry.create_instance(instance_create_info, None))


}

fn get_enabled_extensions(window: &Window) -> Result<Vec<*const i8>, Error> {
    let mut enumerate_required_extensions =
        ash_window::enumerate_required_extensions(window.display_handle().unwrap().as_raw())?.to_vec();

    enumerate_required_extensions.push(KHR_PORTABILITY_ENUMERATION_NAME.as_ptr());

    Ok(enumerate_required_extensions)
}
