use std::sync::Arc;
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    format::Format,
    image::{sampler::Sampler, view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
};

#[derive(Clone)]
pub struct Texture2d {
    pub image: Arc<ImageView>,
    pub sampler: Arc<Sampler>,
}

impl Texture2d {
    pub fn new<T>(
        memory_allocator: Arc<StandardMemoryAllocator>,
        sampler: Arc<Sampler>,
        source: T,
        width: u32,
        height: u32,
    ) -> anyhow::Result<Self>
    where
        T: BufferContents,
    {
        let _buffer = Buffer::from_data(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            source,
        )?;
        let image = Image::new(
            memory_allocator.clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::R8G8B8A8_SRGB,
                extent: [width, height, 1],
                usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
        )?;

        Ok(Self {
            image: ImageView::new_default(image)?,
            sampler,
        })
    }
}
