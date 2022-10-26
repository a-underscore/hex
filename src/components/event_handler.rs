use crate::ecs::{self, Component, GenericEntity, Id, World};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub type EventHandlerCallback =
    dyn FnMut(&GenericEntity, &Rc<RefCell<World>>, &Event<()>, Duration) -> anyhow::Result<()>;

pub struct EventHandler {
    pub callback: Rc<RefCell<EventHandlerCallback>>,
    pub active: bool,
}

impl EventHandler {
    thread_local! {
        pub static ID: Id = ecs::id("event_handler");
    }

    pub fn new<E>(callback: &Rc<RefCell<E>>, active: bool) -> Rc<RefCell<Self>>
    where
        E: FnMut(&GenericEntity, &Rc<RefCell<World>>, &Event<()>, Duration) -> anyhow::Result<()>
            + 'static,
    {
        Rc::new(RefCell::new(Self {
            callback: callback.clone(),
            active,
        }))
    }

    pub fn update(
        &self,
        p: &GenericEntity,
        world: &Rc<RefCell<World>>,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()> {
        if self.active {
            (self.callback.try_borrow_mut()?)(p, world, event, delta)?;
        }

        Ok(())
    }
}

impl Component for EventHandler {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}
