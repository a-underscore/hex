use crate::{
    components::{Camera, Trans},
    world::renderer_manager::Draw,
    Context, Id, World,
};
use parking_lot::RwLock;
use std::sync::Arc;

pub trait Drawable<E>: Send + Sync {
    fn draw(
        self: Arc<Self>,
        entity: E,
        camera: (Id, Arc<RwLock<Camera>>, Arc<RwLock<Trans>>),
        draw: &mut Draw,
        context: Arc<RwLock<Context>>,
        world: Arc<RwLock<World>>,
    ) -> anyhow::Result<()>;
}
