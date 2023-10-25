pub mod proj;

pub use proj::Proj;

use crate::{
    ecs::component_manager::Component,
    math::{Mat4d, Vec2d, Vec3d},
};

#[derive(Clone)]
pub struct Camera {
    proj: Proj,
    matrix: Mat4d,
    pub main: bool,
    pub active: bool,
}

impl Camera {
    pub fn new(proj: Proj, main: bool, active: bool) -> Self {
        let matrix = proj.matrix();

        Self {
            proj,
            matrix,
            main,
            active,
        }
    }

    pub fn perspective(fov: f32, aspect: f32, clip: Vec2d, main: bool, active: bool) -> Self {
        Self::new(Proj::Perspective((fov, aspect, clip)), main, active)
    }

    pub fn ortho(dims: Vec3d, main: bool, active: bool) -> Self {
        Self::new(Proj::Ortho(dims), main, active)
    }

    pub fn proj(&self) -> &Proj {
        &self.proj
    }

    pub fn set_proj(&mut self, proj: Proj) {
        self.proj = proj;

        self.update_matrix()
    }

    pub fn matrix(&self) -> Mat4d {
        self.matrix
    }

    pub fn update_matrix(&mut self) {
        self.matrix = self.proj.matrix();
    }
}

impl Component for Camera {}
