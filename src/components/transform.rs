use crate::{ecs::Component, id};
use cgmath::{Matrix2, Matrix3, Rad, Vector2};

#[derive(Clone)]
pub struct Transform {
    position: Vector2<f32>,
    rotation: Rad<f32>,
    scale: Vector2<f32>,
    transform: Matrix3<f32>,
}

impl Transform {
    pub fn new(position: Vector2<f32>, rotation: Rad<f32>, scale: Vector2<f32>) -> Self {
        Self {
            position,
            rotation,
            scale,
            transform: Self::calculate_transform(position, rotation, scale),
        }
    }

    pub fn position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;

        self.update_transform();
    }

    pub fn rotation(&self) -> Rad<f32> {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Rad<f32>) {
        self.rotation = rotation;

        self.update_transform();
    }

    pub fn scale(&self) -> Vector2<f32> {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vector2<f32>) {
        self.scale = scale;

        self.update_transform();
    }

    pub fn transform(&self) -> Matrix3<f32> {
        self.transform
    }

    fn update_transform(&mut self) {
        self.transform = Self::calculate_transform(self.position, self.rotation, self.scale);
    }

    fn calculate_transform(
        position: Vector2<f32>,
        rotation: Rad<f32>,
        scale: Vector2<f32>,
    ) -> Matrix3<f32> {
        Matrix3::from_translation(position)
            * Matrix3::from(Matrix2::from_angle(rotation))
            * Matrix3::from_nonuniform_scale(scale.x, scale.y)
    }
}

impl Component for Transform {
    fn id() -> usize {
        id!()
    }
}
