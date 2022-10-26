use crate::ecs::{self, Component, GenericEntity, Id, World};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub type EventHandlerCallback =
    dyn FnMut(&GenericEntity, &Rc<RefCell<World>>, &Event<()>, Duration);

pub struct EventHandler<'a> {
    pub callback: &'a mut EventHandlerCallback,
    pub active: bool,
}

impl<'a> EventHandler<'a> {
    thread_local! {
        pub static ID: Id = ecs::id("event_handler");
    }

    pub fn new<E>(callback: &'a mut EventHandlerCallback, active: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { callback, active }))
    }

    pub fn update(
        &mut self,
        p: &GenericEntity,
        world: &Rc<RefCell<World>>,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()> {
        if self.active {
            (self.callback)(p, world, event, delta);
        }

        Ok(())
    }
}

impl<'a> Component for EventHandler<'a> {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}
