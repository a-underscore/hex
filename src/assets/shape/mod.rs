pub mod vertex;

pub use vertex::Vertex;

use crate::math::Vec2d;
use glium::{index::PrimitiveType, Display, VertexBuffer};
use std::rc::Rc;

#[derive(Clone)]
pub struct Shape {
    pub vertices: Rc<VertexBuffer<Vertex>>,
    pub format: PrimitiveType,
}

impl Shape {
    pub fn new(
        display: &Display,
        vertices: &[Vertex],
        format: PrimitiveType,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            vertices: Rc::new(VertexBuffer::immutable(display, vertices)?),
            format,
        })
    }

    pub fn rect(display: &Display, dims: Vec2d) -> anyhow::Result<Self> {
        let vertices = {
            let dims = dims / 2.0;

            [
                Vertex::new(Vec2d::new(-dims.x(), -dims.y()), Default::default()),
                Vertex::new(Vec2d::new(dims.x(), -dims.y()), Vec2d::new(1.0, 0.0)),
                Vertex::new(Vec2d::new(dims.x(), dims.y()), Vec2d::new(1.0, 1.0)),
                Vertex::new(Vec2d::new(-dims.x(), dims.y()), Vec2d::new(0.0, 1.0)),
            ]
        };

        Self::new(display, &vertices, PrimitiveType::TriangleFan)
    }
}
