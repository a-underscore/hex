pub mod system;

pub use system::System;

use super::{ComponentManager, Context, EntityManager, Ev};

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

    pub fn init(
        &mut self,
        scene: &mut Context,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        for s in &mut self.systems {
            s.init(scene, (em, cm))?;
        }

        Ok(())
    }

    pub fn update(
        &mut self,
        ev: &mut Ev,
        scene: &mut Context,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        for s in &mut self.systems {
            s.update(ev, scene, (em, cm))?;
        }

        Ok(())
    }
}
