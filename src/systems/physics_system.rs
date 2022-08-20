use crate::{
    components::{ColliderRect, Transform},
    ecs::{self, Component, Id, System, World},
};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct PhysicsSystem;

impl PhysicsSystem {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {}))
    }
}

impl PhysicsSystem {
    thread_local! {
        pub static ID: Id = ecs::id("physics_system");
    }

    fn update_colliders(&mut self, world: &mut World, delta: Duration) {
        let components = world
            .get_all_with(&[
                &ecs::id(&ColliderRect::get_id()),
                &ecs::id(&Transform::get_id()),
            ])
            .iter()
            .filter_map(|(p, c)| match c.as_slice() {
                [co, t] => Some((p.clone(), (co.clone(), t.clone()))),
                _ => None,
            })
            .collect::<Vec<_>>();

        for (p, ((_, c), (_, t))) in &components {
            if let (Some(c), Some(t)) = (
                c.borrow_mut().as_any_mut().downcast_mut::<ColliderRect>(),
                t.borrow().as_any_ref().downcast_ref::<Transform>(),
            ) {
                c.update(world, p, t, &components, delta);
            }
        }
    }
}

impl Component for PhysicsSystem {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}

impl System for PhysicsSystem {
    fn update(&mut self, world: &mut World, _event: &Event<()>, delta: Duration) {
        self.update_colliders(world, delta);
    }
}
