pub mod system;

pub use system::System;

use crate::{Context, Control, Id, World};
use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};
use threadpool::ThreadPool;

pub struct SystemManager {
    pipelines: HashMap<Id, Vec<Arc<RwLock<Box<dyn System>>>>>,
    pool: ThreadPool,
}

impl SystemManager {
    pub fn new(num_threads: usize) -> Self {
        Self {
            pipelines: Default::default(),
            pool: ThreadPool::new(num_threads),
        }
    }

    pub fn add_gen(&mut self, pid: Id, s: Box<dyn System>) {
        self.pipelines.entry(pid).or_default().push(Arc::new(RwLock::new(s)))
    }

    pub fn add<S: System>(&mut self, pid: Id, s: S) {
        self.add_gen(pid, Box::new(s));
    }

    pub fn rm(&mut self, pid: Id) {
        if let Some(p) = self.pipelines.get_mut(&pid) {
            p.pop();
        }
    }

    pub fn init(
        &mut self,
        context: Arc<RwLock<Context>>,
        world: Arc<RwLock<World>>,
    ) -> anyhow::Result<()> {
        let context = context.clone();
        let world = world.clone();

        for (id, p) in &self.pipelines {
            self.queue((*id, p), |s| {
                s.write().init(context.clone(), world.clone())?;

                Ok(())
            })?;
        }

        Ok(())
    }

    pub fn update(
        &mut self,
        control: Arc<RwLock<Control>>,
        context: Arc<RwLock<Context>>,
        world: Arc<RwLock<World>>,
    ) -> anyhow::Result<()> {
        let control = control.clone();
        let context = context.clone();
        let world = world.clone();

        for (id, p) in &self.pipelines {
            self.queue((*id, p), |s| {
                s.write().update(control.clone(), context.clone(), world.clone())?;

                Ok(())
            })?;
        }

        Ok(())
    }

    fn queue<F: FnOnce(Arc<RwLock<Box<dyn System>>>) -> anyhow::Result<()> + Send + Sync> (
        &mut self,
        ref mut pipeline @ (id, p): (Id, &Vec<Arc<RwLock<Box<dyn System>>>>),
        f: F,
    ) -> anyhow::Result<()> {
        self.pool.execute(|| {
            for s in p {
                f(s.clone()).unwrap();
            }
        });

        Ok(())
    }
}
