use glium::implement_vertex;
use hex_math::Vec2;

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
