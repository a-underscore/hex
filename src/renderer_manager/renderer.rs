use super::{ComponentManager, Context, Draw, EntityManager};
use parking_lot::RwLock;
use std::sync::Arc;

pub trait Renderer: Send + Sync + 'static {
    fn draw(
        &mut self,
        _: &mut Draw,
        _: Arc<RwLock<Context>>,
        _: Arc<RwLock<EntityManager>>,
        _: Arc<RwLock<ComponentManager>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
