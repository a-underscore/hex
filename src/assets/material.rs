use cgmath::Vector4;

#[derive(Clone)]
pub struct Material {
    pub color: Vector4<f32>,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub reflect: f32,
    pub bias: f32,
}

impl Material {
    pub fn new(
        color: Vector4<f32>,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        reflect: f32,
        bias: f32,
    ) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            reflect,
            bias,
        }
    }
}
