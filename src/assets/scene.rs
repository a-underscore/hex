use crate::ecs::{Component, Entity};
use cgmath::Vector4;
use std::{cell::RefCell, rc::Rc};

pub struct Scene {
    pub bg: Vector4<f32>,
    pub camera: Rc<Entity>,
    pub root: Rc<Entity>,
}

impl Scene {
    pub fn new(bg: Vector4<f32>, camera: Rc<Entity>, root: Rc<Entity>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { bg, camera, root }))
    }

    pub fn init(&self) {
        self.root.clone().init(None);
    }

    pub fn update(&self) {
        self.root.clone().update(None);
    }
}
