pub mod system;

pub use system::System;

use crate::{Context, Control, Id, World};
use parking_lot::RwLock;
use rayon::prelude::*;
use std::{collections::HashMap, sync::Arc};

type Pipeline = Arc<RwLock<Vec<Box<dyn System>>>>;

#[derive(Default)]
pub struct SystemManager {
    pipelines: HashMap<Id, Pipeline>,
}

impl SystemManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_gen(&mut self, pid: Id, s: Box<dyn System>) {
        self.pipelines.entry(pid).or_default().write().push(s);
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
        &self,
        context: Arc<RwLock<Context>>,
        world: Arc<RwLock<World>>,
    ) -> anyhow::Result<()> {
        self.par(|(_, p)| {
            for s in &mut *p.write() {
                s.init(context.clone(), world.clone())?;
            }

            Ok(())
        })?;

        Ok(())
    }

    pub fn update(
        &self,
        control: Arc<RwLock<Control>>,
        context: Arc<RwLock<Context>>,
        world: Arc<RwLock<World>>,
    ) -> anyhow::Result<()> {
        self.par(|(_, p)| {
            for s in &mut *p.write() {
                s.update(control.clone(), context.clone(), world.clone())?;
            }

            Ok(())
        })?;

        Ok(())
    }

    fn par<F: Fn((&u32, &Pipeline)) -> anyhow::Result<()> + Send + Sync>(
        &self,
        f: F,
    ) -> anyhow::Result<()> {
        let res: anyhow::Result<Vec<_>> = self.pipelines.par_iter().map(f).collect();

        res?;

        Ok(())
    }
}
