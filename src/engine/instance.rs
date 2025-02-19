use anyhow::Error;
use ash::{vk::InstanceCreateInfo, Entry, Instance};

use super::errors::instance_errors::EntryLoadingError;

static VULKAN_LIBRARY_LOCATION : &str = "/Users/tufan/VulkanSDK/1.3.296.0/macOS/lib/libvulkan.dylib";

pub fn create_instance() -> Result<Instance, Error> {
    let entry = unsafe { match 
        Entry::load_from(VULKAN_LIBRARY_LOCATION) {
            Ok(entry) => entry,
            Err(err) => {
                panic!("{}", EntryLoadingError::InvalidLocation { path: VULKAN_LIBRARY_LOCATION.to_string(), msg: err.to_string()})
            }
        }};
        
    InstanceCreateInfo::default().application_info(application_info)

}
