pub mod entity_manager;
pub mod renderer_manager;
pub mod system_manager;

pub use entity_manager::EntityManager;
pub use renderer_manager::RendererManager;
pub use system_manager::SystemManager;

use parking_lot::{Mutex, RwLock};
use std::sync::Arc;
use threadpool::ThreadPool;

pub struct World {
    pub num_threads: usize,
    pub em: Arc<RwLock<EntityManager>>,
    pub(crate) sm: Arc<RwLock<SystemManager>>,
    pub(crate) rm: Arc<RwLock<RendererManager>>,
    pool: Arc<Mutex<ThreadPool>>,
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
            pool: Arc::new(Mutex::new(ThreadPool::new(num_threads))),
            em,
            sm: Arc::new(RwLock::new(sm)),
            rm: Arc::new(RwLock::new(rm)),
        };

        world.update_num_threads();

        Arc::new(RwLock::new(world))
    }

    pub fn set_num_threads(&mut self, num_threads: usize) {
        self.num_threads = num_threads;
    }

    pub(crate) fn update_num_threads(&self) {
        self.pool.lock().set_num_threads(self.num_threads);
    }
}
