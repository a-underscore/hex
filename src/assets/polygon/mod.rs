pub mod vertex2d;

pub use vertex2d::Vertex2d;

use crate::{ecs::Context, math::Vec2d};

use vulkano::{
    buffer::{subbuffer::Subbuffer, Buffer, BufferCreateInfo, BufferUsage},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};

#[derive(Clone)]
pub struct Shape {
    pub vertices: Subbuffer<[Vertex2d]>,
}

impl Shape {
    pub fn new(context: &mut Context, vertices: &[Vertex2d]) -> anyhow::Result<Self> {
        Ok(Self {
            vertices: Buffer::from_iter(
                context.memory_allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::VERTEX_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                vertices.iter().cloned(),
            )?,
        })
    }

    pub fn rect(context: &mut Context, dims: Vec2d) -> anyhow::Result<Self> {
        let vertices = {
            let dims = dims / 2.0;

            [
                Vertex2d::new(Vec2d::new(-dims.x(), -dims.y()), Default::default()),
                Vertex2d::new(Vec2d::new(dims.x(), -dims.y()), Vec2d::new(1.0, 0.0)),
                Vertex2d::new(Vec2d::new(dims.x(), dims.y()), Vec2d::new(1.0, 1.0)),
                Vertex2d::new(Vec2d::new(-dims.x(), dims.y()), Vec2d::new(0.0, 1.0)),
            ]
        };

        Self::new(context, &vertices)
    }
}
