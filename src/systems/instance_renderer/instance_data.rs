use cgmath::{Matrix4, Vector4};
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct InstanceData {
    pub transform: [[f32; 4]; 4],
    pub color: [f32; 4],
}

impl InstanceData {
    pub fn new(transform: Matrix4<f32>, color: Vector4<f32>) -> Self {
        Self {
            transform: *transform.as_ref(),
            color: *color.as_ref(),
        }
    }
}

implement_vertex!(InstanceData, transform, color);
