use crate::math::Vec4d;

#[derive(Clone)]
pub struct Material {
    pub color: Vec4d,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub reflect: f32,
}

impl Material {
    pub fn new(color: Vec4d, ambient: f32, diffuse: f32, specular: f32, reflect: f32) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            reflect,
        }
    }
}
