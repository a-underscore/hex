use crate::{
    components::Camera,
    ecs::{Component, Entity},
};
use cgmath::Vector4;
use std::{cell::RefCell, rc::Rc};

pub struct SceneData {
    pub bg: Vector4<f32>,
    pub camera: Rc<Camera>,
    pub root: Rc<Entity>,
}

impl SceneData {
    pub fn new(bg: Vector4<f32>, camera: Rc<Camera>, root: Rc<Entity>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { bg, camera, root }))
    }
}

pub struct Scene {
    pub data: Rc<RefCell<SceneData>>,
}

impl Scene {
    pub fn new(bg: Vector4<f32>, camera: Rc<Camera>, root: Rc<Entity>) -> Rc<Self> {
        Rc::new(Self {
            data: SceneData::new(bg, camera, root),
        })
    }

    pub fn init(&self) {
        self.data.borrow().root.clone().init(None);
    }

    pub fn update(&self) {
        self.data.borrow().root.clone().update(None);
    }
}
