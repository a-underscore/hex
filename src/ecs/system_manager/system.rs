use super::{ComponentManager, EntityManager, Ev, Scene};

pub trait System<'a>: 'a {
    fn init(
        &mut self,
        _: &mut Scene,
        _: (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(
        &mut self,
        _: &mut Ev,
        _: &mut Scene,
        _: (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
