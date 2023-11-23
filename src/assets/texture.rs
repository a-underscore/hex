use crate::ecs::Context;
use std::sync::Arc;
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferToImageInfo},
    format::Format,
    image::{sampler::Sampler, view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
    sync::{self, GpuFuture},
};

#[derive(Clone)]
pub struct Texture {
    pub image: Arc<ImageView>,
    pub sampler: Arc<Sampler>,
}

impl Texture {
    pub fn new(
        context: &Context,
        sampler: Arc<Sampler>,
        source: &[u8],
        width: u32,
        height: u32,
    ) -> anyhow::Result<Self> {
        let mut upload = AutoCommandBufferBuilder::primary(
            &context.command_buffer_allocator,
            context.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )?;
        let buffer = Buffer::from_iter(
            context.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            source.iter().cloned(),
        )?;
        let image = Image::new(
            context.memory_allocator.clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::R8G8B8A8_SRGB,
                extent: [width, height, 1],
                usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
        )?;

        upload.copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(buffer, image.clone()))?;

        let command_buffer = upload.build()?;
        let mut previous_frame_end = context.previous_frame_end.write().unwrap();

        *previous_frame_end = Some(
            previous_frame_end
                .take()
                .unwrap()
                .join(
                    sync::now(context.device.clone())
                        .then_execute(context.queue.clone(), command_buffer)?
                        .then_signal_fence_and_flush()?,
                )
                .boxed_send_sync(),
        );

        Ok(Self {
            image: ImageView::new_default(image)?,
            sampler,
        })
    }
}
