use crate::{
    components::{ColliderQuad, Transform},
    ecs::{self, Id, System, World},
};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

thread_local! {
    pub static PHYSICS_SYSTEM_ID: Id = ecs::id("physics_system");
}

pub struct PhysicsSystem {}

impl PhysicsSystem {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {}))
    }
}

impl PhysicsSystem {
    fn update_colliders(&mut self, world: &World, _delta: Duration) {
        let _components = world
            .get_entities()
            .values()
            .filter_map(|e| {
                e.borrow()
                    .get_all(&[&ecs::tid(&ColliderQuad::ID), &ecs::tid(&Transform::ID)])
                    .and_then(|c| match c.as_slice() {
                        [c, t] => Some((c.clone(), t.clone())),

                        _ => None,
                    })
            })
            .collect::<Vec<_>>();
    }
}

impl System for PhysicsSystem {
    fn id(&self) -> Id {
        ecs::tid(&PHYSICS_SYSTEM_ID)
    }

    fn on_update(&mut self, world: &mut World, _event: &Event<()>, delta: Duration) {
        self.update_colliders(world, delta);
    }
}
