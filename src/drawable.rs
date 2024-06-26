use crate::{
    components::{Camera, Trans},
    renderer_manager::Draw,
    ComponentManager, Context, EntityManager, Id,
};
use std::sync::{Arc, RwLock};

pub trait Drawable<E>: Send + Sync {
    fn draw(
        &mut self,
        entity: E,
        camera: (Id, Arc<RwLock<Trans>>, Arc<RwLock<Camera>>),
        draw: &mut Draw,
        context: Arc<RwLock<Context>>,
        em: Arc<RwLock<EntityManager>>,
        cm: Arc<RwLock<ComponentManager>>,
    ) -> anyhow::Result<()>;
}
