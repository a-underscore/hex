pub mod indices;
pub mod vertex;

pub use indices::Indices;
pub use vertex::Vertex;

use glium::{index::PrimitiveType, Display, VertexBuffer};
use std::rc::Rc;

#[derive(Clone)]
pub struct Mesh {
    pub buffer: Rc<(VertexBuffer<Vertex>, Indices)>,
}

impl Mesh {
    pub fn new(
        display: &Display,
        vertices: &[Vertex],
        indices: Option<&[u32]>,
        format: PrimitiveType,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            buffer: Rc::new((
                VertexBuffer::new(display, vertices)?,
                Indices::new(display, indices, format)?,
            )),
        })
    }
}
