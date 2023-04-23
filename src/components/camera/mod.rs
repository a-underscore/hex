pub mod proj;

pub use proj::Proj;

use crate::{
    ecs::{component_manager::Component, Id},
    id,
    math::{Mat4d, Vec2d, Vec3d},
};

#[derive(Clone)]
pub struct Camera {
    proj: Proj,
    view: Mat4d,
    pub active: bool,
}

impl Camera {
    pub fn new(proj: Proj, active: bool) -> Self {
        let view = proj.view();

        Self { proj, view, active }
    }

    pub fn perspective(fov: f32, aspect: f32, clip: Vec2d, active: bool) -> Self {
        Self::new(Proj::Perspective((fov, aspect, clip)), active)
    }

    pub fn ortho(dims: Vec3d, active: bool) -> Self {
        Self::new(Proj::Ortho(dims), active)
    }

    pub fn proj(&self) -> &Proj {
        &self.proj
    }

    pub fn set_proj(&mut self, proj: Proj) {
        self.proj = proj;

        self.update_view()
    }

    pub fn view(&self) -> Mat4d {
        self.view
    }

    pub fn update_view(&mut self) {
        self.view = self.proj.view();
    }
}

impl Component for Camera {
    fn id() -> Id {
        id!()
    }
}
