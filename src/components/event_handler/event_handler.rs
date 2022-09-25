use super::Callback;
use crate::ecs::{self, Component, Entity, Id, World};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

#[derive(Clone)]
pub struct EventHandler {
    pub callback: Rc<RefCell<dyn Callback>>,
    pub active: bool,
}

impl EventHandler {
    thread_local! {
        pub static ID: Id = ecs::id("event_handler");
    }

    pub fn new(callback: Rc<RefCell<dyn Callback>>, active: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { callback, active }))
    }

    pub fn update(
        &self,
        p: (Id, Rc<RefCell<Entity>>),
        world: &Rc<RefCell<World>>,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()> {
        if self.active {
            self.callback
                .try_borrow_mut()?
                .callback(&p, world, event, delta)?;
        }

        Ok(())
    }
}

impl Component for EventHandler {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}
