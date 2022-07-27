use crate::ecs::{self, AsAny, Component, Entity};
use glium::glutin::event::Event;
use std::{any::Any, cell::RefCell, rc::Rc};

pub const EVENT_HANDLER_ID: &str = "event handler";

pub trait Handler: Send + Sync {
    fn handle<'a>(&self, owner: Option<&Entity>, event: &Event<'a, ()>);
}

#[derive(ecs::derive::Component)]
pub struct EventHandler {
    pub id: Rc<String>,
    pub tid: Rc<String>,
    pub handler: Rc<RefCell<Rc<dyn Handler>>>,
}

impl EventHandler {
    pub fn new<'a>(id: Rc<String>, handler: Rc<dyn Handler>) -> Rc<Self> {
        let event_handler = Rc::new(Self {
            id,
            tid: ecs::id(EVENT_HANDLER_ID),
            handler: Rc::new(RefCell::new(handler.clone())),
        });

        event_handler
    }

    pub fn handle(&self, owner: Option<&Entity>, event: &Event<()>) {
        self.handler.borrow().handle(owner, event);
    }
}

impl Component for EventHandler {
    fn id(&self) -> Rc<String> {
        self.id.clone()
    }

    fn tid(&self) -> Rc<String> {
        self.tid.clone()
    }
}
