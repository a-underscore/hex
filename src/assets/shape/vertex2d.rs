use crate::math::Vec2d;

use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

#[derive(BufferContents, Vertex, Copy, Clone)]
#[repr(C)]
pub struct Vertex2d {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
    #[format(R32G32_SFLOAT)]
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
