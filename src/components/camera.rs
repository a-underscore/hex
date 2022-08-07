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
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
    pub active: bool,
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
    ) -> Rc<RefCell<Box<Self>>> {
        Rc::new(RefCell::new(Box::new(Self {
            left,
            right,
            bottom,
            top,
            near,
            far,
            active,
        })))
    }

    pub fn view(&self) -> Matrix4<f32> {
        cgmath::ortho(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.near,
            self.far,
        )
    }
}

impl Component for Camera {
    fn id(&self) -> Id {
        ecs::tid(&CAMERA_ID)
    }
}
