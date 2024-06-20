use super::Sprite;
use crate::{
    components::{Camera, Trans},
    renderer_manager::Draw,
    ComponentManager, Context, EntityManager, Id,
};
use std::sync::{Arc, RwLock};

pub trait Drawable: Send + Sync {
    fn draw(
        &self,
        entity: (Id, Arc<RwLock<Trans>>, Arc<RwLock<Sprite>>),
        camera: (Id, Arc<RwLock<Camera>>, Arc<RwLock<Trans>>),
        context: &Context,
        draw: &mut Draw,
        em: Arc<RwLock<EntityManager>>,
        cm: Arc<RwLock<ComponentManager>>,
    ) -> anyhow::Result<()>;
}
