use super::{ComponentManager, Context, Draw, EntityManager};
use std::sync::{Arc, RwLock};

pub trait Renderer: Send + Sync + 'static {
    fn draw(
        &mut self,
        _: &mut Draw,
        _: &mut Context,
        _: (Arc<RwLock<EntityManager>>, Arc<RwLock<ComponentManager>>),
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
