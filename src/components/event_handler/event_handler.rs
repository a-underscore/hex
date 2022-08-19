use super::EventHandlerCallback;
use crate::ecs::{self, Component, Entity, Id};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct EventHandler {
    pub callback: Rc<RefCell<dyn EventHandlerCallback>>,
}

impl EventHandler {
    thread_local! {
        pub static ID: Id = ecs::id("event_handler");
    }

    pub fn new<C>(callback: &Rc<RefCell<C>>) -> Rc<RefCell<Self>>
    where
        C: EventHandlerCallback,
    {
        Rc::new(RefCell::new(Self {
            callback: callback.clone(),
        }))
    }

    pub fn update(&self, p: (Id, Rc<RefCell<Entity>>), event: &Event<()>, delta: Duration) {
        self.callback.borrow_mut().callback(p, event, delta);
    }
}

impl Component for EventHandler {
    fn get_id(&self) -> Id {
        ecs::tid(&Self::ID)
    }
}
