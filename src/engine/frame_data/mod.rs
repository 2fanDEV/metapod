use anyhow::Error;
use ash::{
    vk::{
        CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, CommandPool,
        CommandPoolCreateFlags, CommandPoolCreateInfo, Instance,
    },
    Device,
};

pub static MAX_FRAME_SIZE: usize = 2;

pub struct FrameData {
    command_pool: CommandPool,
    command_buffer: CommandBuffer,
}

impl FrameData {
    pub fn new(
        device: &Device,
        queue_family_index: u32,
    ) -> Result<FrameData, Error> {
        let create_info = CommandPoolCreateInfo::default()
            .queue_family_index(queue_family_index)
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        let command_pool = unsafe { device.create_command_pool(&create_info, None).unwrap() };

        let command_buffer_allocate_info = CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let command_buffer = unsafe {
            *device
                            .allocate_command_buffers(&command_buffer_allocate_info)
                            .unwrap()
                            .get(0)
                            .unwrap()
        };

        Ok(Self {
            command_pool,
            command_buffer
        })
    }
}
