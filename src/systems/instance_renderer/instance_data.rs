use crate::math::{Mat4d, Vec4d};
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct InstanceData {
    pub transform: [[f32; 4]; 4],
    pub color: [f32; 4],
}

impl InstanceData {
    pub fn new(transform: Mat4d, color: Vec4d) -> Self {
        Self {
            transform: transform.0,
            color: color.0,
        }
    }
}

implement_vertex!(InstanceData, transform, color);
