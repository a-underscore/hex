use crate::{
    ecs::{Component, System, World},
    systems::{DrawingSystem, PhysicsSystem},
    Engine,
};
use cgmath::Vector4;
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct Scene {
    pub bg: Vector4<f32>,
    pub world: Rc<RefCell<World>>,
}

impl Scene {
    pub fn new(bg: Vector4<f32>, world: Rc<RefCell<World>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { bg, world }))
    }

    pub fn default_systems<'a>(
        bg: Vector4<f32>,
        world: Rc<RefCell<World>>,
        engine: Rc<Engine<'static>>,
    ) -> Rc<RefCell<Self>> {
        let scene = Self { bg, world };

        scene.add_default_systems(engine);

        Rc::new(RefCell::new(scene))
    }

    pub fn add_default_systems(&self, engine: Rc<Engine<'static>>) {
        let mut world = self.world.borrow_mut();

        world.add_system(&DrawingSystem::new(engine));
        world.add_system(&PhysicsSystem::new());
    }

    pub fn add_system<S>(&self, system: Rc<RefCell<S>>)
    where
        S: System + Component + 'static,
    {
        self.world.borrow_mut().add_system(&system);
    }

    pub fn init(&self) {
        self.world.borrow_mut().init_systems();
    }

    pub fn update(&self, event: &Event<()>, delta: Duration) {
        self.world.borrow_mut().update_systems(event, delta);
    }
}
