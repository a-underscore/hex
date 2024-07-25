use crate::{
    components::{Camera, Trans},
    renderer_manager::Draw,
    ComponentManager, Context, EntityManager, Id,
};
use parking_lot::RwLock;
use std::sync::Arc;

pub trait Drawable<E>: Send + Sync {
    fn draw(
        &mut self,
        entity: E,
        camera: (Id, Arc<RwLock<Camera>>, Arc<RwLock<Trans>>),
        draw: &mut Draw,
        context: Arc<RwLock<Context>>,
        em: Arc<RwLock<EntityManager>>,
        cm: Arc<RwLock<ComponentManager>>,
    ) -> anyhow::Result<()>;
}
