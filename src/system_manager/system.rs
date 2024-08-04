use crate::{Control, ComponentManager, Context, EntityManager};
use parking_lot::RwLock;
use std::sync::Arc;

pub trait System: Send + Sync + 'static {
    fn init(
        &mut self,
        _: Arc<RwLock<Context>>,
        _: Arc<RwLock<EntityManager>>,
        _: Arc<RwLock<ComponentManager>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(
        &mut self,
        _: Arc<RwLock<Control>>,
        _: Arc<RwLock<Context>>,
        _: Arc<RwLock<EntityManager>>,
        _: Arc<RwLock<ComponentManager>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
