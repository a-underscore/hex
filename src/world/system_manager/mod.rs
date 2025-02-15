pub mod system;

pub use system::System;

use crate::{Context, Control, Id, World};
use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};
use threadpool::ThreadPool;

type Pipeline = Arc<RwLock<Vec<Arc<RwLock<Box<dyn System>>>>>>;

pub struct SystemManager {
    pipelines: HashMap<Id, Pipeline>,
}

impl Default for SystemManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            pipelines: Default::default(),
        }
    }

    pub fn add_gen(&mut self, pid: Id, s: Box<dyn System>) {
        self.pipelines
            .entry(pid)
            .or_default()
            .write()
            .push(Arc::new(RwLock::new(s)))
    }

    pub fn add<S: System>(&mut self, pid: Id, s: S) {
        self.add_gen(pid, Box::new(s));
    }

    pub fn rm(&mut self, pid: Id) {
        if let Some(p) = self.pipelines.get_mut(&pid) {
            p.write().pop();
        }
    }

    pub fn init(
        &mut self,
        context: Arc<RwLock<Context>>,
        world: Arc<RwLock<World>>,
    ) -> anyhow::Result<()> {
        for (id, p) in &self.pipelines {
            let context = context.clone();
            let pool = world.read().pool.clone();
            let world = world.clone();

            self.queue(&pool, (*id, p.clone()), move |s| {
                s.write().init(context.clone(), world.clone())
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
        let pool = world.read().pool.clone();

        for (id, p) in &self.pipelines {
            let control = control.clone();
            let context = context.clone();
            let world = world.clone();

            self.queue(&pool, (*id, p.clone()), move |s| {
                s.write()
                    .update(control.clone(), context.clone(), world.clone())
            })?;
        }

        pool.join();

        Ok(())
    }

    fn queue<F: Fn(Arc<RwLock<Box<dyn System>>>) -> anyhow::Result<()> + Send + Sync + 'static>(
        &self,
        pool: &ThreadPool,
        (_, p): (Id, Pipeline),
        f: F,
    ) -> anyhow::Result<()> {
        let p = p.clone();

        pool.execute(move || {
            for s in &*p.read() {
                f(s.clone()).unwrap();
            }
        });

        Ok(())
    }
}
