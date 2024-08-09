use super::{Context, Draw, World};
use parking_lot::RwLock;
use std::sync::Arc;

pub trait Renderer: Send + Sync + 'static {
    fn draw(
        &self,
        _: &mut Draw,
        _: Arc<RwLock<Context>>,
        _: Arc<RwLock<World>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
