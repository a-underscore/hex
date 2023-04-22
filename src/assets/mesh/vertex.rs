use crate::math::{Vec2d, Vec3d};
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn new(position: Vec3d, normal: Vec3d, uv: Vec2d) -> Self {
        Self {
            position: position.0,
            normal: normal.0,
            uv: uv.0,
        }
    }
}

implement_vertex!(Vertex, position, uv);
