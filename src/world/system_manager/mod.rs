pub mod system;

pub use system::System;

use crate::{Context, Control, Id, World};
use parking_lot::RwLock;
use rayon::prelude::*;
use std::{collections::HashMap, sync::Arc};

#[derive(Default)]
pub struct SystemManager {
    pipelines: HashMap<Id, Vec<Box<dyn System>>>,
}

impl SystemManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_gen(&mut self, pid: Id, s: Box<dyn System>) {
        self.pipelines.entry(pid).or_default().push(s);
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
        self.par(|(_, p)| {
            for s in p {
                s.init(context.clone(), world.clone())?;
            }

            Ok(())
        })?;

        Ok(())
    }

    pub fn update(
        &mut self,
        control: Arc<RwLock<Control>>,
        context: Arc<RwLock<Context>>,
        world: Arc<RwLock<World>>,
    ) -> anyhow::Result<()> {
        self.par(|(_, p)| {
            for s in p {
                s.update(control.clone(), context.clone(), world.clone())?;
            }

            Ok(())
        })?;

        Ok(())
    }

    fn par<F: Fn((&u32, &mut Vec<Box<dyn System>>)) -> anyhow::Result<()> + Send + Sync>(
        &mut self,
        f: F,
    ) -> anyhow::Result<()> {
        let res: anyhow::Result<Vec<_>> = self.pipelines.par_iter_mut().map(f).collect();

        res?;

        Ok(())
    }
}
