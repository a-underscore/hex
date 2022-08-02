use crate::{components::Camera, ecs::Entity};
use cgmath::Vector4;
use std::{cell::RefCell, rc::Rc};

pub struct Scene {
    pub bg: Vector4<f32>,
    pub camera: Rc<Camera>,
    pub root: Rc<Entity>,
}

impl Scene {
    pub fn new(bg: Vector4<f32>, camera: Rc<Camera>, root: Rc<Entity>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { bg, camera, root }))
    }
}
