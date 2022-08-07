use crate::{
    components::Camera,
    ecs::{Component, World},
};
use cgmath::Vector4;
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

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

    pub fn on_init(&self) {
        let world = self.world.borrow();

        for s in world.systems.values() {
            s.clone().on_init(world.root.clone());
        }

        world.root.clone().on_init(None);
    }

    pub fn on_update(&self, event: &Event<()>, delta: Duration) {
        let world = self.world.borrow();

        for s in world.systems.values() {
            s.clone().on_update(world.root.clone(), event, delta);
        }

        world.root.clone().on_update(None, event, delta);
    }
}
