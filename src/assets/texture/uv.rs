use cgmath::Vector2;
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Uv {
    pub uv: [f32; 2],
}

impl Uv {
    pub fn new(uv: Vector2<f32>) -> Self {
        Self { uv: *uv.as_ref() }
    }
}

implement_vertex!(Uv, uv);