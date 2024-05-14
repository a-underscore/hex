use crate::ecs::component_manager::Component;
use crate::nalgebra::{Matrix4, Orthographic3, Vector3};

#[derive(Clone)]
pub struct Camera {
    pub active: bool,
    dimensions: Vector3<f32>,
    proj: Matrix4<f32>,
}

impl Camera {
    pub fn new(dimensions: Vector3<f32>, active: bool) -> Self {
        Self {
            dimensions,
            proj: Self::calculate_proj(dimensions),
            active,
        }
    }

    pub fn dimensions(&self) -> Vector3<f32> {
        self.dimensions
    }

    pub fn set_dimensions(&mut self, dimensions: Vector3<f32>) {
        self.dimensions = dimensions;

        self.update_proj();
    }

    pub fn proj(&self) -> Matrix4<f32> {
        self.proj
    }

    pub fn update_proj(&mut self) {
        self.proj = Self::calculate_proj(self.dimensions);
    }

    pub fn calculate_proj(v: Vector3<f32>) -> Matrix4<f32> {
        let z = v.z;
        let v = v / 2.0;

        Orthographic3::new(-v.x, v.x, -v.y, v.y, -z, 0.0).to_homogeneous()
    }
}

impl Component for Camera {}
