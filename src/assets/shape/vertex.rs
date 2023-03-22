use crate::math::Vec2;
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn new(position: Vec2, uv: Vec2) -> Self {
        Self {
            position: position.0,
            uv: uv.0,
        }
    }
}

implement_vertex!(Vertex, position, uv);
