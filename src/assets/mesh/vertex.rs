use crate::math::Vec3d;
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn new(position: Vec3d, normal: Vec3d) -> Self {
        Self {
            position: position.0,
            normal: normal.0,
        }
    }
}

implement_vertex!(Vertex, position);
