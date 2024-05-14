pub mod system;

pub use system::System;

use super::{ComponentManager, Context, Control, EntityManager, Id};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

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
        self.pipelines
            .entry(pid)
            .or_default()
            .write()
            .unwrap()
            .push(s);
    }

    pub fn add<S>(&mut self, pid: Id, s: S)
    where
        S: System,
    {
        self.add_gen(pid, Box::new(s));
    }

    pub fn rm(&mut self, pid: Id) {
        if let Some(p) = self.pipelines.get_mut(&pid) {
            p.write().unwrap().pop();
        }
    }

    pub fn init(
        &self,
        context: Arc<RwLock<Context>>,
        (em, cm): (Arc<RwLock<EntityManager>>, Arc<RwLock<ComponentManager>>),
    ) -> anyhow::Result<()> {
        let res: anyhow::Result<Vec<_>> = self
            .pipelines
            .par_iter()
            .map(|(_, p)| {
                for s in &mut *p.write().unwrap() {
                    s.init(context.clone(), em.clone(), cm.clone())?;
                }

                Ok(())
            })
            .collect();

        res?;

        Ok(())
    }

    pub fn update(
        &self,
        control: Arc<RwLock<Control>>,
        context: Arc<RwLock<Context>>,
        (em, cm): (Arc<RwLock<EntityManager>>, Arc<RwLock<ComponentManager>>),
    ) -> anyhow::Result<()> {
        let res: anyhow::Result<Vec<_>> = self
            .pipelines
            .par_iter()
            .map(|(_, p)| {
                for s in &mut *p.write().unwrap() {
                    s.update(control.clone(), context.clone(), em.clone(), cm.clone())?;
                }

                Ok(())
            })
            .collect();

        res?;

        Ok(())
    }
}
