use crate::ecs::{self, derive::Component, AsAny, Component, Entity};
use glium::glutin::event::Event;
use std::{any::Any, cell::RefCell, rc::Rc};

thread_local! {
    pub static EVENT_HANDLER_ID: Rc<String> = ecs::id("event_handler");
}

pub trait Handler: Send + Sync {
    fn handle<'a>(&self, owner: Option<&Entity>, event: &Event<'a, ()>);
}

pub struct EventHandlerData {
    pub handler: Rc<dyn Handler>,
}

impl EventHandlerData {
    pub fn new(handler: Rc<dyn Handler>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { handler }))
    }
}

#[derive(Component)]
pub struct EventHandler {
    id: Rc<String>,
    tid: Rc<String>,
    parent: Rc<RefCell<Option<Rc<Entity>>>>,
    pub data: Rc<RefCell<EventHandlerData>>,
}

impl EventHandler {
    pub fn new<'a>(id: Rc<String>, handler: Rc<dyn Handler>) -> Rc<Self> {
        Rc::new(Self {
            id,
            tid: EVENT_HANDLER_ID.with(|id| id.clone()),
            parent: Rc::new(RefCell::new(None)),
            data: EventHandlerData::new(handler),
        })
    }

    pub fn handle(&self, owner: Option<&Entity>, event: &Event<()>) {
        self.data.borrow().handler.handle(owner, event);
    }
}

impl Component for EventHandler {
    fn id(&self) -> Rc<String> {
        self.id.clone()
    }

    fn tid(&self) -> Rc<String> {
        self.tid.clone()
    }

    fn parent(&self) -> Option<Rc<Entity>> {
        self.parent.borrow().clone()
    }

    fn set_parent(&self, parent: Option<Rc<Entity>>) {
        *self.parent.borrow_mut() = parent;
    }
}
