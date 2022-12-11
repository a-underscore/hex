pub mod event_handler_callback;

pub use event_handler_callback::EventHandlerCallback;

use crate::{
    ecs::{self, Component, Entity, Id, Type, World},
    systems::EventSystem,
};
use glium::glutin::event::Event;

#[derive(Clone)]
pub struct EventHandler {
    pub callback: Type<dyn EventHandlerCallback>,
    pub active: bool,
}

impl EventHandler {
    pub fn new(callback: Type<dyn EventHandlerCallback>, active: bool) -> Type<Self> {
        ecs::new(Self { callback, active })
    }

    pub fn update(
        &mut self,
        parent: &(Id, &mut Entity),
        system: &mut EventSystem,
        world: &mut World,
        event: &Event<()>,
    ) -> anyhow::Result<()> {
        self.callback
            .clone()
            .try_borrow_mut()?
            .callback(self, parent, system, world, event)?;

        Ok(())
    }
}

impl Component for EventHandler {
    fn id() -> Id {
        ecs::id("event_handler")
    }
}
