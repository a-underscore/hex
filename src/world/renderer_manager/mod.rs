pub mod draw;
pub mod renderer;

pub use draw::Draw;
pub use renderer::Renderer;

use crate::{Context, World};

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
        &self,
        draw: &mut Draw,
        context: Arc<RwLock<Context>>,
        world: Arc<RwLock<World>>,
    ) -> anyhow::Result<()> {
        for r in &self.renderers {
            r.draw(draw, context.clone(), world.clone())?;
        }

        Ok(())
    }
}
