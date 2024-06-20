use crate::{
    components::{Camera, Trans},
    renderer_manager::Draw,
    ComponentManager, Context, EntityManager, Id,
};
use std::sync::{Arc, RwLock};

pub trait Drawable<E>: Send + Sync {
    fn draw(
        &self,
        entity: E,
        camera: (Id, Arc<RwLock<Trans>>, Arc<RwLock<Camera>>),
        context: &Context,
        draw: &mut Draw,
        em: &EntityManager,
        cm: &ComponentManager,
    ) -> anyhow::Result<()>;
}
