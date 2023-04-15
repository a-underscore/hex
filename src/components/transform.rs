use crate::{
    ecs::{component_manager::Component, Id},
    id,
    math::{Mat3d, Vec2d},
};

#[derive(Clone)]
pub struct Transform {
    position: Vec2d,
    rotation: f32,
    scale: Vec2d,
    matrix: Mat3d,
    pub active: bool,
}

impl Transform {
    pub fn new(position: Vec2d, rotation: f32, scale: Vec2d, active: bool) -> Self {
        Self {
            position,
            rotation,
            scale,
            matrix: Self::calculate_matrix(position, rotation, scale),
            active,
        }
    }

    pub fn position(&self) -> Vec2d {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2d) {
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

    pub fn scale(&self) -> Vec2d {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vec2d) {
        self.scale = scale;

        self.update_matrix();
    }

    pub fn matrix(&self) -> Mat3d {
        self.matrix
    }

    pub fn update_matrix(&mut self) {
        self.matrix = Self::calculate_matrix(self.position, self.rotation, self.scale);
    }

    pub fn calculate_matrix(position: Vec2d, rotation: f32, scale: Vec2d) -> Mat3d {
        Mat3d::translation(position) * Mat3d::rotation(rotation) * Mat3d::scale(scale)
    }
}

impl Component for Transform {
    fn id() -> Id {
        id!()
    }
}
