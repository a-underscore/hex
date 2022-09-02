use crate::{
    components::{Collider, Transform},
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

    fn update_colliders(
        &mut self,
        world: &mut World,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()> {
        let components = world
            .get_all_with(&[
                &ecs::id(&Collider::get_id()),
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
                c.try_borrow_mut()?.as_any_mut().downcast_mut::<Collider>(),
                t.try_borrow()?.as_any_ref().downcast_ref::<Transform>(),
            ) {
                if let Err(e) = c.update(world, p, t, &components, event, delta) {
                    println!("{}", e);
                }
            }
        }

        Ok(())
    }
}

impl Component for PhysicsSystem {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}

impl System for PhysicsSystem {
    fn update(&mut self, world: &mut World, event: &Event<()>, delta: Duration) {
        let _ = self.update_colliders(world, event, delta);
    }
}
