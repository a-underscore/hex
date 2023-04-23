use crate::math::Vec2d;
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Uv {
    pub uv: [f32; 2],
}

impl Uv {
    pub fn new(uv: Vec2d) -> Self {
        Self { uv: uv.0 }
    }
}

implement_vertex!(Uv, uv);
