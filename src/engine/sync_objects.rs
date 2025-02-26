use anyhow::Error;
use ash::{vk::{Fence, FenceCreateFlags, FenceCreateInfo, Semaphore, SemaphoreCreateFlags, SemaphoreCreateInfo}, Device};

pub fn create_fence(device: &Device, flags: FenceCreateFlags) -> Result<Fence, Error>{
    let create_info = FenceCreateInfo::default().flags(flags);
    unsafe { Ok(device.create_fence(&create_info, None).unwrap()) }
} 

pub fn create_semaphore(device: &Device) -> Result<Semaphore, Error>{
    let create_info= SemaphoreCreateInfo::default().flags(SemaphoreCreateFlags::empty());
    unsafe { Ok(device.create_semaphore(&create_info, None).unwrap()) }
}
