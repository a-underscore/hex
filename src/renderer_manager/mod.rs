pub mod draw;
pub mod renderer;

pub use draw::Draw;
pub use renderer::Renderer;

use crate::{ComponentManager, Context, EntityManager};

use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Default)]
pub struct RendererManager {
    renderers: Vec<Box<dyn Renderer>>,
}

impl RendererManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_gen(&mut self, r: Box<dyn Renderer>) {
        self.renderers.push(r);
    }

    pub fn add<R: Renderer>(&mut self, r: R) {
        self.add_gen(Box::new(r));
    }

    pub fn rm(&mut self) {
        self.renderers.pop();
    }

    pub fn draw(
        &mut self,
        draw: &mut Draw,
        context: Arc<RwLock<Context>>,
        em: Arc<RwLock<EntityManager>>,
        cm: Arc<RwLock<ComponentManager>>,
    ) -> anyhow::Result<()> {
        for r in &mut self.renderers {
            r.draw(draw, context.clone(), em.clone(), cm.clone())?;
        }

        Ok(())
    }
}
