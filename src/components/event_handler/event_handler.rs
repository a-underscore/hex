use super::EventHandlerCallback;
use crate::ecs::{self, Component, Entity, Id, World};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct EventHandler {
    pub callback: Rc<RefCell<dyn EventHandlerCallback>>,
    pub active: bool,
}

impl EventHandler {
    thread_local! {
        pub static ID: Id = ecs::id("event_handler");
    }

    pub fn new<C>(callback: &Rc<RefCell<C>>, active: bool) -> Rc<RefCell<Self>>
    where
        C: EventHandlerCallback,
    {
        Rc::new(RefCell::new(Self {
            callback: callback.clone(),
            active,
        }))
    }

    pub fn update(
        &self,
        world: &mut World,
        p: (Id, Rc<RefCell<Entity>>),
        event: &Event<()>,
        delta: Duration,
    ) {
        if self.active {
            self.callback.borrow_mut().callback(world, p, event, delta);
        }
    }
}

impl Component for EventHandler {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}
