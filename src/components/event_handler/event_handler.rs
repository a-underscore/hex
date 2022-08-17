use super::EventHandlerCallback;
use crate::ecs::{self, Component, Entity, Id};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc};

pub struct EventHandler {
    pub callback: Rc<RefCell<dyn EventHandlerCallback>>,
}

impl EventHandler {
    thread_local! {
        pub static ID: Id = ecs::id("event_handler");
    }

    pub fn new(callback: Rc<RefCell<dyn EventHandlerCallback>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { callback }))
    }

    pub fn update(&self, p: (Id, Rc<RefCell<Entity>>), event: &Event<()>) {
        self.callback.borrow_mut().callback(p, event);
    }
}

impl Component for EventHandler {
    fn get_id(&self) -> Id {
        ecs::tid(&Self::ID)
    }
}
