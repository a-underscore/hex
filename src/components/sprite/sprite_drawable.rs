use super::{fragment, vertex, Drawable, SpriteEntity};
use crate::{
    components::{Camera, Trans},
    renderer_manager::Draw,
    Context, EntityManager, Id,
};
use parking_lot::RwLock;
use std::sync::Arc;
use vulkano::{
    buffer::{
        allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo},
        BufferUsage,
    },
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    memory::allocator::MemoryTypeFilter,
    padded::Padded,
    pipeline::{Pipeline, PipelineBindPoint},
};

pub struct SpriteDrawable;

impl SpriteDrawable {
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self))
    }
}

impl Drawable<SpriteEntity> for SpriteDrawable {
    fn draw(
        &mut self,
        (_, s, t): SpriteEntity,
        (_, c, ct): (Id, Arc<RwLock<Camera>>, Arc<RwLock<Trans>>),
        (_, builder, recreate_swapchain): &mut Draw,
        context: Arc<RwLock<Context>>,
        _: Arc<RwLock<EntityManager>>,
    ) -> anyhow::Result<()> {
        let context = context.read();
        let c = c.read();
        let ct = ct.read();

        if *recreate_swapchain {
            let mut s = s.write();

            s.recreate_pipeline(&context)?;
        }

        let s = s.read();
        let t = t.read();
        let (pipeline, _, _) = &s.pipeline;

        builder.bind_pipeline_graphics(pipeline.clone())?;

        let view = {
            let layout = pipeline.layout().set_layouts().first().unwrap();
            let subbuffer_allocator = SubbufferAllocator::new(
                context.memory_allocator.clone(),
                SubbufferAllocatorCreateInfo {
                    buffer_usage: BufferUsage::UNIFORM_BUFFER,
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
            );
            let subbuffer = subbuffer_allocator.allocate_sized()?;

            *subbuffer.write()? = vertex::View {
                z: Padded(c.calculate_z(s.layer)),
                transform: <[[f32; 3]; 3]>::from(t.matrix()).map(Padded),
                camera_transform: <[[f32; 3]; 3]>::from(ct.matrix()).map(Padded),
                camera_proj: c.proj().into(),
            };

            PersistentDescriptorSet::new(
                &context.descriptor_set_allocator,
                layout.clone(),
                [WriteDescriptorSet::buffer(0, subbuffer)],
                [],
            )?
        };
        let texture = {
            let layout = pipeline.layout().set_layouts().get(1).unwrap();

            PersistentDescriptorSet::new(
                &context.descriptor_set_allocator,
                layout.clone(),
                [
                    WriteDescriptorSet::sampler(0, s.texture.sampler.clone()),
                    WriteDescriptorSet::image_view(1, s.texture.image.clone()),
                ],
                [],
            )?
        };
        let color = {
            let layout = pipeline.layout().set_layouts().get(2).unwrap();
            let subbuffer_allocator = SubbufferAllocator::new(
                context.memory_allocator.clone(),
                SubbufferAllocatorCreateInfo {
                    buffer_usage: BufferUsage::UNIFORM_BUFFER,
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
            );
            let subbuffer = subbuffer_allocator.allocate_sized()?;

            *subbuffer.write()? = fragment::Color {
                color: s.color.into(),
            };

            PersistentDescriptorSet::new(
                &context.descriptor_set_allocator,
                layout.clone(),
                [WriteDescriptorSet::buffer(0, subbuffer)],
                [],
            )?
        };

        builder
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                pipeline.layout().clone(),
                0,
                view.clone(),
            )?
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                pipeline.layout().clone(),
                1,
                texture.clone(),
            )?
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                pipeline.layout().clone(),
                2,
                color.clone(),
            )?
            .bind_vertex_buffers(0, s.shape.vertices.clone())?
            .draw(s.shape.vertices.len() as u32, 1, 0, 0)?;

        Ok(())
    }
}
