use cgmath::Vector2;
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn new(position: Vector2<f32>, uv: Vector2<f32>) -> Self {
        Self {
            position: position.into(),
            uv: uv.into(),
        }
    }
}

implement_vertex!(Vertex, position, uv);
