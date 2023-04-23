use glium::{
    index::{IndicesSource, NoIndices, PrimitiveType},
    Display, IndexBuffer,
};

pub enum Indices {
    Indexed(Box<IndexBuffer<u32>>),
    Ordered(NoIndices),
}

impl Indices {
    pub fn new(
        display: &Display,
        indices: Option<&[u32]>,
        format: PrimitiveType,
    ) -> anyhow::Result<Self> {
        Ok(match indices {
            Some(i) => Self::Indexed(Box::new(IndexBuffer::new(display, format, i)?)),
            None => Self::Ordered(NoIndices(format)),
        })
    }

    pub fn source(&self) -> IndicesSource<'_> {
        match self {
            Self::Indexed(indices) => indices.as_ref().into(),
            Self::Ordered(indices) => indices.into(),
        }
    }
}
