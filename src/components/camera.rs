use crate::ecs::component_manager::Component;
use crate::nalgebra::{Matrix4, Orthographic3, Vector2};

#[derive(Clone)]
pub struct Camera {
    pub active: bool,
    dimensions: Vector2<f32>,
    max_layer: u32,
    proj: Matrix4<f32>,
}

impl Camera {
    pub fn new(dimensions: Vector2<f32>, max_layer: u32, active: bool) -> Self {
        Self {
            dimensions,
            max_layer,
            proj: Self::calculate_proj(dimensions, max_layer),
            active,
        }
    }

    pub fn dimensions(&self) -> Vector2<f32> {
        self.dimensions
    }

    pub fn set_dimensions(&mut self, dimensions: Vector2<f32>) {
        self.dimensions = dimensions;

        self.update_proj();
    }

    pub fn max_layer(&self) -> u32 {
        self.max_layer
    }

    pub fn set_max_layer(&mut self, max_layer: u32) {
        self.max_layer = max_layer;

        self.update_proj();
    }

    pub fn proj(&self) -> Matrix4<f32> {
        self.proj
    }

    pub fn update_proj(&mut self) {
        self.proj = Self::calculate_proj(self.dimensions, self.max_layer);
    }

    pub fn calculate_proj(v: Vector2<f32>, max_layer: u32) -> Matrix4<f32> {
        let z = max_layer as f32;
        let v = v / 2.0;

        Orthographic3::new(-v.x, v.x, -v.y, v.y, -z, z).to_homogeneous()
    }
}

impl Component for Camera {}
