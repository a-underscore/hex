pub mod ev;
pub mod system;

pub use ev::Ev;
pub use system::System;

use crate::ecs::{component_manager::ComponentManager, entity_manager::EntityManager};
use glium::Display;

#[derive(Default)]
pub struct SystemManager<'a> {
    pub systems: Vec<Box<dyn System<'a>>>,
}

impl<'a> SystemManager<'a> {
    pub fn add<S>(&mut self, s: S)
    where
        S: System<'a>,
    {
        self.systems.push(Box::new(s));
    }

    pub fn rm(&mut self) {
        self.systems.pop();
    }

    pub fn init(
        &mut self,
        display: &Display,
        entity_manager: &mut EntityManager,
        component_manager: &mut ComponentManager,
    ) -> anyhow::Result<()> {
        for s in &mut self.systems {
            s.init(display, entity_manager, component_manager)?;
        }

        Ok(())
    }

    pub fn update(
        &mut self,
        display: &Display,
        event: &mut Ev,
        entity_manager: &mut EntityManager,
        component_manager: &mut ComponentManager,
    ) -> anyhow::Result<()> {
        for s in &mut self.systems {
            s.update(display, event, entity_manager, component_manager)?;
        }

        Ok(())
    }
}
