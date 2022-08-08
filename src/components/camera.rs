use crate::ecs::{self, Component, Id};
use cgmath::Matrix4;
use std::{cell::RefCell, rc::Rc};

thread_local! {
    pub static CAMERA_ID: Id = ecs::id("camera");
}

pub struct CameraData {}

impl CameraData {
    pub fn new() -> Rc<RefCell<Box<Self>>> {
        Rc::new(RefCell::new(Box::new(Self {})))
    }
}

pub struct Camera {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
    view: Matrix4<f32>,
    active: bool,
}

impl Camera {
    pub fn new(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
        active: bool,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            left,
            right,
            bottom,
            top,
            near,
            far,
            view: Self::calculate_view(left, right, bottom, top, near, far),
            active,
        }))
    }

    pub fn get_left(&self) -> f32 {
        self.left
    }

    pub fn set_left(&mut self, left: f32) {
        self.left = left;

        self.update_view();
    }

    pub fn get_right(&self) -> f32 {
        self.right
    }

    pub fn set_right(&mut self, right: f32) {
        self.right = right;

        self.update_view();
    }

    pub fn get_bottom(&self) -> f32 {
        self.bottom
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.bottom = bottom;

        self.update_view();
    }

    pub fn get_top(&self) -> f32 {
        self.top
    }

    pub fn set_top(&mut self, top: f32) {
        self.top = top;

        self.update_view();
    }

    pub fn get_near(&self) -> f32 {
        self.near
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;

        self.update_view();
    }

    pub fn get_far(&self) -> f32 {
        self.far
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;

        self.update_view();
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        self.view
    }

    pub fn get_active(&self) -> bool {
        self.active
    }

    fn calculate_view(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Matrix4<f32> {
        cgmath::ortho(left, right, bottom, top, near, far)
    }

    fn update_view(&mut self) {
        Self::calculate_view(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.near,
            self.far,
        );
    }
}

impl Component for Camera {
    fn id(&self) -> Id {
        ecs::tid(&CAMERA_ID)
    }
}
