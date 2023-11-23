pub mod system;

pub use system::System;

use super::{ComponentManager, Context, Control, Draw, EntityManager, Id};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Default)]
pub struct SystemManager {
    pipelines: HashMap<Id, Vec<Arc<RwLock<dyn System>>>>,
}

impl SystemManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_gen(&mut self, pid: Id, s: Arc<RwLock<dyn System>>) {
        self.pipelines
            .entry(pid)
            .or_insert(Default::default())
            .push(s);
    }

    pub fn add<S>(&mut self, pid: Id, s: S)
    where
        S: System,
    {
        self.add_gen(pid, Arc::new(RwLock::new(s)));
    }

    pub fn rm(&mut self, pid: Id) {
        if let Some(p) = self.pipelines.get_mut(&pid) {
            p.pop();
        }
    }

    pub fn init(
        &mut self,
        context: Arc<RwLock<Context>>,
        (em, cm): (Arc<RwLock<EntityManager>>, Arc<RwLock<ComponentManager>>),
    ) -> anyhow::Result<()> {
        let res: anyhow::Result<Vec<_>> = self
            .pipelines
            .par_iter()
            .map(|(_, p)| {
                for s in p {
                    s.write()
                        .unwrap()
                        .init(context.clone(), (em.clone(), cm.clone()))?;
                }

                Ok(())
            })
            .collect();

        res?;

        Ok(())
    }

    pub fn update(
        &mut self,
        control: Arc<RwLock<Control>>,
        context: Arc<RwLock<Context>>,
        (em, cm): (Arc<RwLock<EntityManager>>, Arc<RwLock<ComponentManager>>),
    ) -> anyhow::Result<()> {
        let res: anyhow::Result<Vec<_>> = self
            .pipelines
            .par_iter()
            .map(|(_, p)| {
                for s in p {
                    s.write().unwrap().update(
                        control.clone(),
                        context.clone(),
                        (em.clone(), cm.clone()),
                    )?;
                }

                Ok(())
            })
            .collect();

        res?;

        Ok(())
    }
}
