use glium::{
    index::{IndicesSource, NoIndices, PrimitiveType},
    Display, IndexBuffer,
};

pub enum Indices {
    Indexed(IndexBuffer<u32>),
    Ordered(NoIndices),
}

impl Indices {
    pub fn new(
        display: &Display,
        indices: Option<&[u32]>,
        format: PrimitiveType,
    ) -> anyhow::Result<Self> {
        Ok(match indices {
            Some(i) => Self::Indexed(IndexBuffer::new(display, format, i)?),
            None => Self::Ordered(NoIndices(format)),
        })
    }

    pub fn source<'a>(&'a self) -> IndicesSource<'a> {
        match self {
            Self::Indexed(indices) => indices.into(),
            Self::Ordered(indices) => indices.into(),
        }
    }
}
