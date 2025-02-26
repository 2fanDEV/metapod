use anyhow::Error;
use ash::{
    vk::{
        CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, CommandPool, CommandPoolCreateFlags, CommandPoolCreateInfo, Fence, Instance, Semaphore
    },
    Device,
};

use super::command_buffers::{create_command_buffer, create_command_pool};


pub struct FrameData {
    pub command_pool: CommandPool,
    pub command_buffer: CommandBuffer,
    pub swapchain_semaphore: Semaphore,
    pub render_semaphore: Semaphore,
    pub render_fence: Fence
}

impl FrameData {
    pub fn new(
        device: &Device,
        queue_family_index: u32,
        render_semaphore : Semaphore, 
        swapchain_semaphore: Semaphore,
        render_fence: Fence
    ) -> Result<FrameData, Error> {
        let command_pool = create_command_pool(device, queue_family_index)?;
        let command_buffer = create_command_buffer(device, command_pool)?;
        Ok(Self {
            command_pool,
            command_buffer,
            render_fence,
            render_semaphore,
            swapchain_semaphore
        })
    }
}
