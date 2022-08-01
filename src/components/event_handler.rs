use crate::ecs::{self, AsAny, Component, Entity};
use glium::glutin::event::Event;
use std::{any::Any, cell::RefCell, rc::Rc};

thread_local! {
    pub static EVENT_HANDLER_ID: Rc<String> = ecs::id("event_handler");
}

pub trait Handler: Send + Sync {
    fn handle<'a>(&self, owner: Option<&Entity>, event: &Event<'a, ()>);
}

pub struct EventHandlerData {
    pub id: Rc<String>,
    pub handler: Rc<dyn Handler>,
}

impl EventHandlerData {
    pub fn new(id: Rc<String>, handler: Rc<dyn Handler>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { id, handler }))
    }
}

#[derive(ecs::derive::Component)]
pub struct EventHandler {
    pub tid: Rc<String>,
    data: Rc<RefCell<EventHandlerData>>,
}

impl EventHandler {
    pub fn new<'a>(id: Rc<String>, handler: Rc<dyn Handler>) -> Rc<Self> {
        Rc::new(Self {
            tid: EVENT_HANDLER_ID.with(|id| id.clone()),
            data: EventHandlerData::new(id, handler),
        })
    }

    pub fn handle(&self, owner: Option<&Entity>, event: &Event<()>) {
        self.data.borrow().handler.handle(owner, event);
    }
}

impl Component for EventHandler {
    fn id(&self) -> Rc<String> {
        self.data.borrow().id.clone()
    }

    fn tid(&self) -> Rc<String> {
        self.tid.clone()
    }
}
