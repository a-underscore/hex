use crate::cid;
use hecs::component_manager::Component;
use hex_math::{Mat3, Vec2};

#[derive(Clone)]
pub struct Transform {
    position: Vec2,
    rotation: f32,
    scale: Vec2,
    matrix: Mat3,
    pub active: bool,
}

impl Transform {
    pub fn new(position: Vec2, rotation: f32, scale: Vec2, active: bool) -> Self {
        Self {
            position,
            rotation,
            scale,
            matrix: Self::calculate_matrix(&position, rotation, &scale),
            active,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
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

    pub fn scale(&self) -> Vec2 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;

        self.update_matrix();
    }

    pub fn matrix(&self) -> Mat3 {
        self.matrix
    }

    fn update_matrix(&mut self) {
        self.matrix = Self::calculate_matrix(&self.position, self.rotation, &self.scale);
    }

    fn calculate_matrix(position: &Vec2, rotation: f32, scale: &Vec2) -> Mat3 {
        Mat3::translation(position) * Mat3::rotation(rotation) * Mat3::scale(scale)
    }
}

impl Component for Transform {
    fn id() -> usize {
        cid!()
    }
}
