use crate::{glium::implement_vertex, math::Vec2};

#[derive(Copy, Clone)]
pub struct Vertex2 {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}

impl Vertex2 {
    pub fn new(position: Vec2, uv: Vec2) -> Self {
        Self {
            position: position.0,
            uv: uv.0,
        }
    }
}

implement_vertex!(Vertex2, position, uv);
