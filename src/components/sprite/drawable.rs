use crate::{
    components::{Camera, Sprite, Trans},
    renderer_manager::Draw,
    Context,
};
use std::sync::{Arc, RwLock};

pub trait Drawable: Send + Sync {
    fn draw(
        &self,
        sprite: Arc<RwLock<Sprite>>,
        trans: Arc<RwLock<Trans>>,
        camera: (Arc<RwLock<Camera>>, Arc<RwLock<Trans>>),
        context: &Context,
        draw: &mut Draw,
    ) -> anyhow::Result<()>;
}
