use crate::{components::Camera, ecs::World};
use cgmath::Vector4;
use std::{cell::RefCell, rc::Rc};

pub struct Scene {
    pub bg: Vector4<f32>,
    pub camera: Rc<Camera>,
    pub world: Rc<RefCell<World>>,
}

impl Scene {
    pub fn new(
        bg: Vector4<f32>,
        camera: Rc<Camera>,
        world: Rc<RefCell<World>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { bg, camera, world }))
    }
}
