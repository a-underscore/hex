use crate::{
    id,
    ecs::{Component, Entities, Components},
};
use glium::glutin::event::Event;
use std::collections::HashMap;

pub struct EventHandler {
    pub callback: Box<dyn Fn(usize, &mut Entities, &Event<()>)>,
    pub active: bool,
}

impl EventHandler {
    pub fn new<F>(callback: Box<F>, active: bool) -> Self 
    where F: Fn(usize, &mut Entities, &Event<()>) + 'static {
        Self { callback, active }
    }

    pub fn update(
        &mut self,
        parent: usize,
        entities: &mut HashMap<usize, Components>,
        event: &Event<()>,
    ) -> anyhow::Result<()> {
        (self.callback)(parent, entities, event);

        Ok(())
    }
}

impl Component for EventHandler {
    fn id() -> usize {
        id!()
    }
}
