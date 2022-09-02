use crate::{
    components::event_handler::EventHandler,
    ecs::{self, Component, Id, System, World},
};
use glium::glutin::event::Event;
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
    time::Duration,
};

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
    fn update(&mut self, world: &mut World, event: &Event<()>, delta: Duration) {
        for (p, (_, c)) in world.get_all(&EventHandler::get_id()) {
            if let Some(c) = c.try_borrow().ok().and_then(|c| {
                Ref::filter_map(c, |c| c.as_any_ref().downcast_ref::<EventHandler>()).ok()
            }) {
                if let Err(e) = c.update(world, p, event, delta) {
                    println!("{}", e);
                }
            }
        }
    }
}
