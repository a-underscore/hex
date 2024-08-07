use nalgebra::{Matrix3, Vector2};
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone)]
pub struct Trans {
    position: Vector2<f32>,
    rotation: f32,
    scale: Vector2<f32>,
    matrix: Matrix3<f32>,
}

impl Trans {
    pub fn new(position: Vector2<f32>, rotation: f32, scale: Vector2<f32>) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            position,
            rotation,
            scale,
            matrix: Self::calculate_matrix(position, rotation, scale),
        }))
    }

    pub fn position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;

        self.update_matrix();
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;

        self.update_matrix();
    }

    pub fn scale(&self) -> Vector2<f32> {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vector2<f32>) {
        self.scale = scale;

        self.update_matrix();
    }

    pub fn matrix(&self) -> Matrix3<f32> {
        self.matrix
    }

    fn update_matrix(&mut self) {
        self.matrix = Self::calculate_matrix(self.position, self.rotation, self.scale);
    }

    fn calculate_matrix(
        position: Vector2<f32>,
        rotation: f32,
        scale: Vector2<f32>,
    ) -> Matrix3<f32> {
        Matrix3::new_translation(&position)
            * Matrix3::new_nonuniform_scaling(&scale)
            * Matrix3::new_rotation(rotation)
    }
}
