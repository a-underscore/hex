pub mod vertex;

pub use vertex::Vertex;

use crate::math::{Vec2d, Vec3d};
use glium::{index::PrimitiveType, Display, IndexBuffer, VertexBuffer};
use std::rc::Rc;

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Rc<VertexBuffer<Vertex>>,
    pub indices: Rc<IndexBuffer<u32>>,
}

impl Mesh {
    pub fn new(
        display: &Display,
        vertices: &[Vertex],
        indices: &[u32],
        format: PrimitiveType,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            vertices: Rc::new(VertexBuffer::immutable(display, vertices)?),
            indices: Rc::new(IndexBuffer::immutable(display, format, indices)?),
        })
    }

    pub fn rect(display: &Display, dims: Vec2d) -> anyhow::Result<Self> {
        static INDICES: &[u32] = &[0, 1, 2, 0, 2, 3];

        let vertices = {
            let dims = dims / 2.0;

            [
                Vertex::new(
                    Vec3d::new(-dims.x(), -dims.y(), 0.0),
                    Vec3d::new(0.0, 0.0, -1.0),
                    Default::default(),
                ),
                Vertex::new(
                    Vec3d::new(dims.x(), -dims.y(), 0.0),
                    Vec3d::new(0.0, 0.0, -1.0),
                    Vec2d::new(1.0, 0.0),
                ),
                Vertex::new(
                    Vec3d::new(dims.x(), dims.y(), 0.0),
                    Vec3d::new(0.0, 0.0, -1.0),
                    Vec2d::new(1.0, 1.0),
                ),
                Vertex::new(
                    Vec3d::new(-dims.x(), dims.y(), 0.0),
                    Vec3d::new(0.0, 0.0, -1.0),
                    Vec2d::new(0.0, 1.0),
                ),
            ]
        };

        Self::new(display, &vertices, INDICES, PrimitiveType::TrianglesList)
    }
}
