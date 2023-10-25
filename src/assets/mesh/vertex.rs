use cgmath::Vector3;
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn new(position: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Self {
            position: *position.as_ref(),
            normal: *normal.as_ref(),
        }
    }
}

implement_vertex!(Vertex, position, normal);
