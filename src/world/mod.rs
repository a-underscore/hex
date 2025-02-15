pub mod entity_manager;
pub mod renderer_manager;
pub mod system_manager;

pub use entity_manager::EntityManager;
pub use renderer_manager::RendererManager;
pub use system_manager::SystemManager;

use parking_lot::RwLock;
use std::sync::Arc;
use threadpool::ThreadPool;

pub struct World {
    pub num_threads: usize,
    pub em: Arc<RwLock<EntityManager>>,
    pub pool: ThreadPool,
    pub(crate) sm: Arc<RwLock<SystemManager>>,
    pub(crate) rm: Arc<RwLock<RendererManager>>,
}

impl World {
    pub fn new(
        num_threads: usize,
        em: Arc<RwLock<EntityManager>>,
        sm: SystemManager,
        rm: RendererManager,
    ) -> Arc<RwLock<Self>> {
        let world = Self {
            num_threads,
            pool: ThreadPool::new(num_threads),
            em,
            sm: Arc::new(RwLock::new(sm)),
            rm: Arc::new(RwLock::new(rm)),
        };

        Arc::new(RwLock::new(world))
    }
}
