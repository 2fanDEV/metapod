use anyhow::Error;
use ash::vk::{AccessFlags2, CommandBuffer, Image, ImageAspectFlags, ImageLayout, ImageMemoryBarrier2KHR, ImageSubresourceRange, PipelineStageFlags2, REMAINING_ARRAY_LAYERS, REMAINING_MIP_LEVELS};
use log::LevelFilter;

pub fn transition_image(
    command_buffer: CommandBuffer,
    image: Image,
    current_layout: ImageLayout,
    new_layout: ImageLayout,
) -> Result<(), Error> {
    let image_aspect_flag = match new_layout == ImageLayout::DEPTH_ATTACHMENT_OPTIMAL {
        true => ImageAspectFlags::DEPTH,
        false => ImageAspectFlags::COLOR
    };

    
    let sub_resource_range = image_sub_resource_range(image_aspect_flag);
    
    let image_barrier = ImageMemoryBarrier2KHR::default()
        .src_stage_mask(PipelineStageFlags2::ALL_COMMANDS)
        .src_access_mask(AccessFlags2::MEMORY_WRITE)
        .dst_stage_mask(PipelineStageFlags2::ALL_COMMANDS)
        .dst_access_mask(AccessFlags2::MEMORY_WRITE | AccessFlags2::MEMORY_READ)
        .old_layout(current_layout)
        .new_layout(new_layout)
        .subresource_range(sub_resource_range)
        ;

    Ok(())
}

    fn image_sub_resource_range(aspect_flag: ImageAspectFlags) -> ImageSubresourceRange {
        ImageSubresourceRange::default()
            .aspect_mask(aspect_flag)
            .base_mip_level(0)
            .level_count(REMAINING_MIP_LEVELS)
            .base_array_layer(0)
            .layer_count(REMAINING_ARRAY_LAYERS)
    }

