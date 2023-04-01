pub mod system;

pub use system::System;

use super::{Ev, Scene, World};

#[derive(Default)]
pub struct SystemManager<'a> {
    pub systems: Vec<Box<dyn System<'a>>>,
}

impl<'a> SystemManager<'a> {
    pub fn add<S>(&mut self, s: S)
    where
        S: System<'a>,
    {
        self.systems.push(Box::new(s));
    }

    pub fn rm(&mut self) {
        self.systems.pop();
    }

    pub fn init(&mut self, scene: &mut Scene, world: &mut World) -> anyhow::Result<()> {
        for s in &mut self.systems {
            s.init(scene, world)?;
        }

        Ok(())
    }

    pub fn update(
        &mut self,
        event: &mut Ev,
        scene: &mut Scene,
        world: &mut World,
    ) -> anyhow::Result<()> {
        for s in &mut self.systems {
            s.update(event, scene, world)?;
        }

        Ok(())
    }
}
