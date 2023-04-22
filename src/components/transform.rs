use crate::{
    ecs::{component_manager::Component, Id},
    id,
    math::{Mat4d, Vec3d},
};

#[derive(Clone)]
pub struct Transform {
    position: Vec3d,
    rotation: Vec3d,
    scale: Vec3d,
    matrix: Mat4d,
    pub active: bool,
}

impl Transform {
    pub fn new(position: Vec3d, rotation: Vec3d, scale: Vec3d, active: bool) -> Self {
        Self {
            position,
            rotation,
            scale,
            matrix: Self::calculate_matrix(position, rotation, scale),
            active,
        }
    }

    pub fn position(&self) -> Vec3d {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3d) {
        self.position = position;

        self.update_matrix();
    }

    pub fn rotation(&self) -> Vec3d {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Vec3d) {
        self.rotation = rotation;

        self.update_matrix();
    }

    pub fn scale(&self) -> Vec3d {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vec3d) {
        self.scale = scale;

        self.update_matrix();
    }

    pub fn matrix(&self) -> Mat4d {
        self.matrix
    }

    pub fn update_matrix(&mut self) {
        self.matrix = Self::calculate_matrix(self.position, self.rotation, self.scale);
    }

    pub fn calculate_matrix(position: Vec3d, rotation: Vec3d, scale: Vec3d) -> Mat4d {
        Mat4d::translation(position)
            * Mat4d::rotation_x(rotation.x())
            * Mat4d::rotation_y(rotation.y())
            * Mat4d::rotation_z(rotation.z())
            * Mat4d::scale(scale)
    }
}

impl Component for Transform {
    fn id() -> Id {
        id!()
    }
}
