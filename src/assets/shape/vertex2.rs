use nalgebra::Vector2;
use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

#[derive(BufferContents, Vertex, Copy, Clone)]
#[repr(C)]
pub struct Vertex2 {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
    #[format(R32G32_SFLOAT)]
    pub uv: [f32; 2],
}

impl Vertex2 {
    pub fn new(position: Vector2<f32>, uv: Vector2<f32>) -> Self {
        Self {
            position: position.into(),
            uv: uv.into(),
        }
    }
}
