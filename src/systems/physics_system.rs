use crate::{
    components::{ColliderRect, Transform},
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
        let components = world
            .get_all_with(&[&ecs::tid(&ColliderRect::ID), &ecs::tid(&Transform::ID)])
            .iter()
            .filter_map(|((id, e), c)| match c.as_slice() {
                [(_, co), (_, t)] => Some(((id.clone(), e.clone()), (co.clone(), t.clone()))),
                _ => None,
            })
            .collect::<Vec<_>>();

        for (p, (c, t)) in &components {
            if let (Some(c), Some(t)) = (
                c.borrow_mut().as_any_mut().downcast_mut::<ColliderRect>(),
                t.borrow().as_any_ref().downcast_ref::<Transform>(),
            ) {
                c.update(&p, t, &components);
            }
        }
    }
}

impl System for PhysicsSystem {
    fn id(&self) -> Id {
        ecs::tid(&PHYSICS_SYSTEM_ID)
    }

    fn update(&mut self, world: &mut World, _event: &Event<()>, delta: Duration) {
        self.update_colliders(world, delta);
    }
}
