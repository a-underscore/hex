use crate::ecs::component_manager::Component;
use cgmath::{Matrix4, Rad, Vector3};

#[derive(Clone)]
pub struct Transform {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
    matrix: Matrix4<f32>,
    pub active: bool,
}

impl Transform {
    pub fn new(
        position: Vector3<f32>,
        rotation: Vector3<f32>,
        scale: Vector3<f32>,
        active: bool,
    ) -> Self {
        Self {
            position,
            rotation,
            scale,
            matrix: Self::calculate_matrix(position, rotation, scale),
            active,
        }
    }

    pub fn position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;

        self.update_matrix();
    }

    pub fn rotation(&self) -> Vector3<f32> {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Vector3<f32>) {
        self.rotation = rotation;

        self.update_matrix();
    }

    pub fn scale(&self) -> Vector3<f32> {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;

        self.update_matrix();
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        self.matrix
    }

    pub fn update_matrix(&mut self) {
        self.matrix = Self::calculate_matrix(self.position, self.rotation, self.scale);
    }

    pub fn calculate_matrix(
        position: Vector3<f32>,
        rotation: Vector3<f32>,
        scale: Vector3<f32>,
    ) -> Matrix4<f32> {
        Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z)
            * Matrix4::from_angle_z(Rad(rotation.z))
            * Matrix4::from_angle_y(Rad(rotation.y))
            * Matrix4::from_angle_x(Rad(rotation.x))
            * Matrix4::from_translation(position)
    }
}

impl Component for Transform {}
