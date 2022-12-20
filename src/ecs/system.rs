use super::{ComponentManager, EntityManager};
use glium::{glutin::event::Event, Display};

pub trait System<'a>: 'a {
    fn init(
        &mut self,
        _: &mut EntityManager,
        _: &mut ComponentManager,
        _: &Display,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(
        &mut self,
        _: &mut EntityManager,
        _: &mut ComponentManager,
        _: &Display,
        _: &Event<()>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
