pub mod vertex2;

pub use vertex2::Vertex2;

use crate::ecs::Context;
use nalgebra::Vector2;
use vulkano::{
    buffer::{subbuffer::Subbuffer, Buffer, BufferCreateInfo, BufferUsage},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};

#[derive(Clone)]
pub struct Shape {
    pub vertices: Subbuffer<[Vertex2]>,
}

impl Shape {
    pub fn new(context: &Context, vertices: &[Vertex2]) -> anyhow::Result<Self> {
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

    pub fn rect(context: &Context, dims: Vector2<f32>) -> anyhow::Result<Self> {
        let vertices = {
            let dims = dims / 2.0;

            [
                Vertex2::new(Vector2::new(-dims.x, -dims.y), Default::default()),
                Vertex2::new(Vector2::new(dims.x, -dims.y), Vector2::new(1.0, 0.0)),
                Vertex2::new(Vector2::new(dims.x, dims.y), Vector2::new(1.0, 1.0)),
                Vertex2::new(Vector2::new(-dims.x, dims.y), Vector2::new(0.0, 1.0)),
            ]
        };

        Self::new(context, &vertices)
    }
}
