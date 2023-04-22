pub mod projection;

pub use projection::Projection;

use crate::{
    ecs::{component_manager::Component, Id},
    id,
    math::{Mat4d, Vec2d, Vec3d},
};

#[derive(Clone)]
pub struct Camera {
    view: Mat4d,
    proj: Projection,
    pub active: bool,
}

impl Camera {
    pub fn perspective(fov: f32, aspect: f32, clip: Vec2d, active: bool) -> Self {
        let proj = Projection::Perspective(fov, aspect, clip);

        Self {
            view: proj.view(),
            proj,
            active,
        }
    }

    pub fn ortho(dims: Vec3d, active: bool) -> Self {
        let proj = Projection::Ortho(dims);

        Self {
            view: proj.view(),
            proj,
            active,
        }
    }

    pub fn proj(&self) -> &Projection {
        &self.proj
    }

    pub fn set_proj(&mut self, proj: Projection) {
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
