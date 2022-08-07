use crate::{ecs::World, systems::DrawingSystem, Engine};
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

    pub fn default_systems(
        bg: Vector4<f32>,
        world: Rc<RefCell<World>>,
        engine: Rc<RefCell<Engine<'static>>>,
    ) -> Rc<RefCell<Self>> {
        let scene = Scene::new(bg, world);
        let drawing_system = DrawingSystem::new(engine.clone());

        {
            let scene = scene.borrow();
            let mut world = scene.world.borrow_mut();

            world.add_system(&drawing_system);
        }

        scene
    }

    pub fn init(&self) {
        self.world.borrow_mut().init();
    }

    pub fn update(&self) {
        self.world.borrow_mut().update();
    }
}
