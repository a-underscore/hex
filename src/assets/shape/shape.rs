use super::Vertex;
use cgmath::{Vector2, Zero};
use glium::{index::PrimitiveType, Display, IndexBuffer, VertexBuffer};
use std::{cell::RefCell, rc::Rc};

pub struct Shape {
    pub vertices: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u32>,
}

impl Shape {
    pub fn new(
        display: &Display,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> anyhow::Result<Rc<RefCell<Self>>> {
        Ok(Rc::new(RefCell::new(Self {
            vertices: VertexBuffer::new(display, vertices)?,
            indices: IndexBuffer::immutable(display, PrimitiveType::TrianglesList, &indices)?,
        })))
    }

    pub fn new_rect(display: &Display, dims: Vector2<f32>) -> anyhow::Result<Rc<RefCell<Self>>> {
        let indices = [0, 1, 2, 1, 3, 2];
        let vertices = {
            let dims = dims / 2.0;

            [
                Vertex::new(Vector2::new(-dims.x, -dims.y), Vector2::zero()),
                Vertex::new(Vector2::new(-dims.x, dims.y), Vector2::new(0.0, 1.0)),
                Vertex::new(Vector2::new(dims.x, -dims.y), Vector2::new(1.0, 0.0)),
                Vertex::new(Vector2::new(dims.x, dims.y), Vector2::new(1.0, 1.0)),
            ]
        };

        Self::new(display, &vertices, &indices)
    }
}
