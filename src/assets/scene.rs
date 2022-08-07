use crate::ecs::{System, World};
use cgmath::Vector4;
use std::{cell::RefCell, rc::Rc};

pub struct Scene {
    pub bg: Vector4<f32>,
    pub world: Rc<RefCell<World>>,
}

impl Scene {
    pub fn new(bg: Vector4<f32>, world: Rc<RefCell<World>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { bg, world }))
    }

    pub fn add_system<S>(&self, system: Rc<S>)
    where
        S: System,
    {
        self.world.borrow_mut().add_system(&system);
    }

    pub fn init(&self) {
        self.world.borrow_mut().init();
    }

    pub fn update(&self) {
        self.world.borrow_mut().update();
    }
}
