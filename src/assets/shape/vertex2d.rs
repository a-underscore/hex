use crate::math::Vec2d;
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex2d {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}

impl Vertex2d {
    pub fn new(position: Vec2d, uv: Vec2d) -> Self {
        Self {
            position: position.0,
            uv: uv.0,
        }
    }
}

implement_vertex!(Vertex2d, position, uv);
