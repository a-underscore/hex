pub mod vertex;

pub use vertex::Vertex;

use cgmath::{Vector2, Zero};
use glium::{index::PrimitiveType, Display, IndexBuffer, VertexBuffer};
use std::rc::Rc;

pub static INDICES: [u32; 6] = [0, 1, 2, 1, 3, 2];

pub struct Shape {
    pub vertices: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u32>,
}

impl Shape {
    pub fn new(
        display: &Rc<Display>,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> anyhow::Result<Self> {
        Ok(Self {
            vertices: VertexBuffer::new(display.as_ref(), vertices)?,
            indices: IndexBuffer::immutable(
                display.as_ref(),
                PrimitiveType::TrianglesList,
                indices,
            )?,
        })
    }

    pub fn rect(display: &Rc<Display>, dims: Vector2<f32>) -> anyhow::Result<Self> {
        let vertices = {
            let dims = dims / 2.0;

            [
                Vertex::new(Vector2::new(-dims.x, -dims.y), Vector2::zero()),
                Vertex::new(Vector2::new(-dims.x, dims.y), Vector2::new(0.0, 1.0)),
                Vertex::new(Vector2::new(dims.x, -dims.y), Vector2::new(1.0, 0.0)),
                Vertex::new(Vector2::new(dims.x, dims.y), Vector2::new(1.0, 1.0)),
            ]
        };

        Self::new(display, &vertices, &INDICES)
    }
}
