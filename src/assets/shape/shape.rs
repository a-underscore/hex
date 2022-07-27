use super::vertex::Vertex;
use glium::{index::PrimitiveType, Display, IndexBuffer, VertexBuffer};
use std::rc::Rc;

pub struct Shape {
    pub vertices: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u32>,
}

impl Shape {
    pub fn new(
        display: &Display,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> anyhow::Result<Rc<Self>> {
        Ok(Rc::new(Self {
            vertices: VertexBuffer::new(display, vertices)?,
            indices: IndexBuffer::immutable(display, PrimitiveType::TrianglesList, &indices)?,
        }))
    }
}
