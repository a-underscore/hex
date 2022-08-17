use crate::{
    components::event_handler::EventHandler,
    ecs::{self, Id, System, World},
};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct EventSystem;

impl EventSystem {
    thread_local! {
        pub static ID: Id = ecs::id("event_system");
    }

    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self))
    }
}

impl System for EventSystem {
    fn id(&self) -> Id {
        ecs::tid(&Self::ID)
    }

    fn update(&mut self, world: &mut World, event: &Event<()>, _delta: Duration) {
        for (p, (_, c)) in world.get_all(&ecs::tid(&EventHandler::ID)) {
            if let Some(c) = c.borrow().as_any_ref().downcast_ref::<EventHandler>() {
                c.update(p, event);
            }
        }
    }
}
