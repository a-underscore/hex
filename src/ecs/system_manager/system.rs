use super::{ComponentManager, Context, EntityManager};

pub trait System: 'static {
    fn init(
        &mut self,
        _: &mut Context,
        _: (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(
        &mut self,
        _: &mut Ev,
        _: &mut Context,
        _: (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
