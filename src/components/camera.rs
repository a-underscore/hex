use crate::{
    ecs::{component_manager::Component, Id},
    id,
    math::Mat4d,
};

#[derive(Clone)]
pub struct Camera {
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
    view: Mat4d,
    pub active: bool,
}

impl Camera {
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32, active: bool) -> Self {
        Self {
            fov,
            aspect,
            near,
            far,
            view: Mat4d::perspective(fov, aspect, near, far),
            active,
        }
    }

    pub fn ortho(aspect: f32, near: f32, far: f32, active: bool) -> Self {
        let fov = 0.0;

        Self {
            fov,
            aspect,
            near,
            far,
            view: Mat4d::perspective(fov, aspect, near, far),
            active,
        }
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn set_dimensions(&mut self, fov: f32) {
        self.fov = fov;

        self.update_view();
    }

    pub fn aspect(&self) -> f32 {
        self.aspect
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;

        self.update_view();
    }

    pub fn near(&self) -> f32 {
        self.near
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;

        self.update_view();
    }

    pub fn far(&self) -> f32 {
        self.far
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;

        self.update_view();
    }

    pub fn view(&self) -> Mat4d {
        self.view
    }

    pub fn update_view(&mut self) {
        self.view = Mat4d::perspective(self.fov, self.aspect, self.near, self.far);
    }
}

impl Component for Camera {
    fn id() -> Id {
        id!()
    }
}
