use anyhow::Error;
use ash::{
    vk::{
        CommandBuffer, CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferLevel,
        CommandBufferUsageFlags, CommandPool, CommandPoolCreateFlags, CommandPoolCreateInfo,
    },
    Device,
};

pub fn create_command_pool(device: &Device, queue_family_index: u32) -> Result<CommandPool, Error> {
    let create_info = CommandPoolCreateInfo::default()
        .queue_family_index(queue_family_index)
        .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
    Ok(unsafe { device.create_command_pool(&create_info, None).unwrap() })
}

pub fn create_command_buffer(
    device: &Device,
    command_pool: CommandPool,
) -> Result<CommandBuffer, Error> {
    let command_buffer_allocate_info = CommandBufferAllocateInfo::default()
        .command_pool(command_pool)
        .level(CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);

    Ok(unsafe {
        *device
            .allocate_command_buffers(&command_buffer_allocate_info)
            .unwrap()
            .get(0)
            .unwrap()
    })
}

pub fn begin_command_buffer(
    device: &Device,
    command_buffer: CommandBuffer,
    flags: CommandBufferUsageFlags,
) -> Result<(), Error> {
    let begin_info = CommandBufferBeginInfo::default().flags(flags);

    unsafe {
        device
            .begin_command_buffer(command_buffer, &begin_info)
            .unwrap()
    };
    Ok(())
}
