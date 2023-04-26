pub mod proj_type;

pub use proj_type::ProjType;

use crate::{
    ecs::{component_manager::Component, Id},
    id,
    math::{Mat4d, Vec2d, Vec3d},
};

#[derive(Clone)]
pub struct Camera {
    proj_type: ProjType,
    proj: Mat4d,
    pub main: bool,
    pub active: bool,
}

impl Camera {
    pub fn new(proj_type: ProjType, main: bool, active: bool) -> Self {
        let proj = proj_type.proj();

        Self {
            proj_type,
            proj,
            main,
            active,
        }
    }

    pub fn perspective(fov: f32, aspect: f32, clip: Vec2d, main: bool, active: bool) -> Self {
        Self::new(ProjType::Perspective((fov, aspect, clip)), main, active)
    }

    pub fn ortho(dims: Vec3d, main: bool, active: bool) -> Self {
        Self::new(ProjType::Ortho(dims), main, active)
    }

    pub fn proj_type(&self) -> &ProjType {
        &self.proj_type
    }

    pub fn set_proj_type(&mut self, proj_type: ProjType) {
        self.proj_type = proj_type;

        self.update_proj()
    }

    pub fn proj(&self) -> Mat4d {
        self.proj
    }

    pub fn update_proj(&mut self) {
        self.proj = self.proj_type.proj();
    }
}

impl Component for Camera {
    fn id() -> Id {
        id!()
    }
}
