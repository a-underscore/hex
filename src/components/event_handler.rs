use crate::{
    ecs::{Component, Manager},
    id,
};
use glium::glutin::event::Event;
use std::rc::Rc;

pub struct EventHandler {
    pub callback: Rc<dyn Fn(usize, &mut Manager, &Event<()>)>,
    pub active: bool,
}

impl EventHandler {
    pub fn new<F>(callback: Rc<F>, active: bool) -> Self
    where
        F: Fn(usize, &mut Manager, &Event<()>) + 'static,
    {
        Self { callback, active }
    }

    pub fn update(
        &self,
        parent: usize,
        manager: &mut Manager,
        event: &Event<()>,
    ) -> anyhow::Result<()> {
        (self.callback)(parent, manager, event);

        Ok(())
    }
}

impl Component for EventHandler {
    fn id() -> usize {
        id!()
    }
}
