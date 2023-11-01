pub mod system;

pub use system::System;

use super::{ComponentManager, Context, EntityManager, Ev};

#[derive(Default)]
pub struct SystemManager {
    pub systems: Vec<Box<dyn System>>,
}

impl SystemManager {
    pub fn add<S>(&mut self, s: S)
    where
        S: System,
    {
        self.systems.push(Box::new(s));
    }

    pub fn rm(&mut self) {
        self.systems.pop();
    }

    pub fn init(
        &mut self,
        context: &mut Context,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        for s in &mut self.systems {
            s.init(context, (em, cm))?;
        }

        Ok(())
    }

    pub fn update(
        &mut self,
        ev: &mut Ev,
        context: &mut Context,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        for s in &mut self.systems {
            s.update(ev, context, (em, cm))?;
        }

        Ok(())
    }
}
