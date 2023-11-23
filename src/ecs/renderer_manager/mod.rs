pub mod renderer;

pub use renderer::Renderer;

use super::{ComponentManager, Context, Control, Draw, EntityManager, Id};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Default)]
pub struct RendererManager {
    renderers: Vec<Arc<RwLock<dyn Renderer>>>,
}

impl RendererManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_gen(&mut self, r: Arc<RwLock<dyn Renderer>>) {
        self.renderers.push(r);
    }

    pub fn add<R>(&mut self, r: R)
    where
        R: Renderer,
    {
        self.add_gen(Arc::new(RwLock::new(r)));
    }

    pub fn rm(&mut self) {
        self.renderers.pop();
    }

    pub fn draw(
        &mut self,
        draw: &mut Draw,
        context: &mut Context,
        (em, cm): (Arc<RwLock<EntityManager>>, Arc<RwLock<ComponentManager>>),
    ) -> anyhow::Result<()> {
        for r in &self.renderers {
            r.write()
                .unwrap()
                .draw(draw, context, (em.clone(), cm.clone()))?;
        }

        Ok(())
    }
}
