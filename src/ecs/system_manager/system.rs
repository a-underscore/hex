use super::Ev;
use crate::ecs::{component_manager::ComponentManager, entity_manager::EntityManager};
use glium::Display;

pub trait System<'a>: 'a {
    fn init(
        &mut self,
        _: &Display,
        _: &mut EntityManager,
        _: &mut ComponentManager,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(
        &mut self,
        _: &Display,
        _: &mut Ev,
        _: &mut EntityManager,
        _: &mut ComponentManager,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
