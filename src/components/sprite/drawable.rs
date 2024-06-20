use crate::{
    components::{Camera, Sprite, Trans},
    renderer_manager::Draw,
    Context, Id,
};
use std::sync::{Arc, RwLock};

pub trait Drawable: Send + Sync {
    fn draw(
        &self,
        id: Id,
        sprite: Arc<RwLock<Sprite>>,
        trans: Arc<RwLock<Trans>>,
        camera: (Id, Arc<RwLock<Camera>>, Arc<RwLock<Trans>>),
        context: &Context,
        draw: &mut Draw,
    ) -> anyhow::Result<()>;
}
