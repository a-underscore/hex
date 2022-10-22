use crate::{
    components::event_handler::EventHandler,
    ecs::{self, Component, Id, System, World},
};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

#[derive(Clone)]
pub struct EventSystem;

impl EventSystem {
    thread_local! {
        pub static ID: Id = ecs::id("event_system");
    }

    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self))
    }
}

impl Component for EventSystem {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}

impl System for EventSystem {
    fn update(
        &mut self,
        world: &Rc<RefCell<World>>,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()> {
        for (p, c) in world
            .try_borrow()
            .map(|w| w.clone())?
            .get_all_ref::<EventHandler>()
        {
            c.update(p, world, event, delta)?;
        }

        Ok(())
    }
}
