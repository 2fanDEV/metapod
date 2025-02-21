use std::ffi::CStr;

use ash::{
    vk::{
        ApplicationInfo, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
        DebugUtilsMessengerCreateInfoEXT, InstanceCreateFlags, InstanceCreateInfo,
        EXT_DEBUG_UTILS_NAME, KHR_PORTABILITY_ENUMERATION_NAME,
        KHR_PORTABILITY_SUBSET_SPEC_VERSION,
    },
    Entry, Instance,
};
use winit::{raw_window_handle::HasDisplayHandle, window::Window};

use super::debugger::debug_callback;
use crate::engine::errors::instance_errors::InstanceCreationError;

static VULKAN_LIBRARY_LOCATION: &str = "/Users/tufan/VulkanSDK/1.3.296.0/macOS/lib/libvulkan.dylib";
static ENGINE_NAME: &CStr = c"Metapod";
static APP_NAME: &CStr = c"METAPOD";

pub fn create_instance(window: &Window) -> Result<(Entry, Instance), InstanceCreationError> {
    let entry = unsafe {
        match Entry::load_from(VULKAN_LIBRARY_LOCATION) {
            Ok(entry) => entry,
            Err(err) => {
                panic!(
                    "{}",
                    InstanceCreationError::EntryInvalidLocation {
                        path: VULKAN_LIBRARY_LOCATION.to_string(),
                        msg: err.to_string()
                    }
                )
            }
        }
    };

    let application_info = ApplicationInfo::default()
        .engine_name(ENGINE_NAME)
        .engine_version(1)
        .application_version(1)
        .application_name(APP_NAME);

    let mut enabled_extension_names = get_enabled_extensions(window)?;

    match check_validation_layer_support(&entry).unwrap() {
        true => enabled_extension_names.push(EXT_DEBUG_UTILS_NAME.as_ptr()),
        false => return Err(InstanceCreationError::ValidationCheck),
    }

    let mut debug_create_info = DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | DebugUtilsMessageSeverityFlagsEXT::WARNING
                | DebugUtilsMessageSeverityFlagsEXT::INFO
                | DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            DebugUtilsMessageTypeFlagsEXT::GENERAL
                | DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(debug_callback));

    let instance_create_info = InstanceCreateInfo::default()
        .enabled_extension_names(&enabled_extension_names)
        .flags(InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR)
        .application_info(&application_info)
        .push_next(&mut debug_create_info);

    let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };
    Ok((entry, instance))
}

fn check_validation_layer_support(entry: &Entry) -> Result<bool, &str> {
    let validation_layers = vec!["VK_LAYER_KHRONOS_validation"];
    unsafe {
        let available_layers = entry.enumerate_instance_layer_properties().unwrap();
        for layer in validation_layers {
            for available_layer in available_layers.iter() {
                if layer.eq(available_layer
                    .layer_name_as_c_str()
                    .unwrap()
                    .to_str()
                    .unwrap())
                {
                    return Ok(true);
                }
            }
        }
    };
    Err("Validation Layers are not present on this machine")
}

fn get_enabled_extensions(window: &Window) -> Result<Vec<*const i8>, InstanceCreationError> {
    let mut enumerate_required_extensions =
        ash_window::enumerate_required_extensions(window.display_handle().unwrap().as_raw())
            .unwrap()
            .to_vec();

    enumerate_required_extensions.push(KHR_PORTABILITY_ENUMERATION_NAME.as_ptr());
    enumerate_required_extensions.push(EXT_DEBUG_UTILS_NAME.as_ptr());
    Ok(enumerate_required_extensions)
}
