use super::Ev;
use crate::ecs::world::World;

pub trait System<'a>: 'a {
    fn init(&mut self, _: &mut World) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(&mut self, _: &mut Ev, _: &mut World) -> anyhow::Result<()> {
        Ok(())
    }
}
