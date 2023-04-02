pub mod vertex2;

pub use vertex2::Vertex2;

use crate::{
    glium::{index::PrimitiveType, Display, VertexBuffer},
    math::Vec2,
};
use std::rc::Rc;

#[derive(Clone)]
pub struct Shape {
    pub vertices: Rc<VertexBuffer<Vertex2>>,
    pub format: PrimitiveType,
}

impl Shape {
    pub fn new(
        display: &Display,
        vertices: &[Vertex2],
        format: PrimitiveType,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            vertices: Rc::new(VertexBuffer::immutable(display, vertices)?),
            format,
        })
    }

    pub fn rect(display: &Display, dims: Vec2) -> anyhow::Result<Self> {
        let vertices = {
            let dims = dims / 2.0;

            [
                Vertex2::new(Vec2::new(-dims.x(), -dims.y()), Default::default()),
                Vertex2::new(Vec2::new(dims.x(), -dims.y()), Vec2::new(1.0, 0.0)),
                Vertex2::new(Vec2::new(dims.x(), dims.y()), Vec2::new(1.0, 1.0)),
                Vertex2::new(Vec2::new(-dims.x(), dims.y()), Vec2::new(0.0, 1.0)),
            ]
        };

        Self::new(display, &vertices, PrimitiveType::TriangleFan)
    }
}
