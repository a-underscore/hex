pub mod entity_manager;
pub mod renderer_manager;
pub mod system_manager;

pub use entity_manager::EntityManager;
pub use renderer_manager::RendererManager;
pub use system_manager::SystemManager;

use parking_lot::RwLock;
use std::sync::Arc;

pub struct World {
    pub em: Arc<RwLock<EntityManager>>,
    pub(crate) sm: Arc<RwLock<SystemManager>>,
    pub(crate) rm: Arc<RwLock<RendererManager>>,
}

impl World {
    pub fn new(
        em: Arc<RwLock<EntityManager>>,
        sm: SystemManager,
        rm: RendererManager,
    ) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            em,
            sm: Arc::new(RwLock::new(sm)),
            rm: Arc::new(RwLock::new(rm)),
        }))
    }
}
