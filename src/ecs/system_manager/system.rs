use super::{Ev, Scene, World};

pub trait System<'a>: 'a {
    fn init(&mut self, _: &mut Scene, _: &mut World) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(&mut self, _: &mut Ev, _: &mut Scene, _: &mut World) -> anyhow::Result<()> {
        Ok(())
    }
}
