use std::ffi::c_void;

use ash::{vk::{DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT, DebugUtilsMessengerCallbackDataEXT, DebugUtilsMessengerCreateInfoEXT, DebugUtilsMessengerEXT}, Entry, Instance};
use log::{error, info, warn};

pub fn setup_debugger(entry: &Entry, instance: &Instance) -> (ash::ext::debug_utils::Instance, DebugUtilsMessengerEXT) {
    let debug_create_info = DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
           DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
           DebugUtilsMessageSeverityFlagsEXT::WARNING |
           DebugUtilsMessageSeverityFlagsEXT::INFO |
           DebugUtilsMessageSeverityFlagsEXT:: ERROR
        ).message_type(
        DebugUtilsMessageTypeFlagsEXT::GENERAL |
        DebugUtilsMessageTypeFlagsEXT::VALIDATION |
        DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
        )
        .pfn_user_callback(Some(debug_callback));
    
    let debug_instance = ash::ext::debug_utils::Instance::new(entry, instance);
    let debug_messenger = unsafe { debug_instance.create_debug_utils_messenger(&debug_create_info, None).unwrap() };
    (debug_instance, debug_messenger)
}

pub unsafe extern "system" fn debug_callback(
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_type: DebugUtilsMessageTypeFlagsEXT,
        callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
        user_data: *mut c_void,
    ) -> u32 {
        unsafe {
            let p_callback_data = *callback_data;
            let message_id_name = p_callback_data
                .message_id_name_as_c_str()
                .unwrap()
                .to_string_lossy();
            let message_id_number = p_callback_data.message_id_number;
            let message = p_callback_data
                .message_as_c_str()
                .unwrap()
                .to_string_lossy();

            match message_severity {
                DebugUtilsMessageSeverityFlagsEXT::WARNING => {
                    warn!(
                        "{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n"
                    );
                }
                DebugUtilsMessageSeverityFlagsEXT::ERROR => {
                    error!(
                        "{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n"
                    )
                }
                _ => {
                    info!(
                        "{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n"
                    );
                } 
            }
        }
        0
    }

