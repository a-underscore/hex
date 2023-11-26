use super::{ComponentManager, Context, Draw, EntityManager};

pub trait Renderer: Send + Sync + 'static {
    fn draw(
        &mut self,
        _: &mut Draw,
        _: &mut Context,
        _: (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
