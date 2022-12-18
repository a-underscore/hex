use super::{Manager, System};
use glium::glutin::event::Event;

#[derive(Default)]
pub struct World<'a, 'b> {
    pub manager: Manager<'a>,
    pub systems: Vec<Box<dyn System<'b>>>,
}

impl<'a, 'b> World<'a, 'b> {
    pub fn add_s<S>(&mut self, s: S)
    where
        S: System<'b>,
    {
        self.systems.push(Box::new(s));
    }
}

impl<'a> World<'a, 'a> {
    pub fn init(&mut self) -> anyhow::Result<()> {
        for s in &mut self.systems {
            s.init(&mut self.manager)?;
        }

        Ok(())
    }

    pub fn update(&mut self, event: &Event<()>) -> anyhow::Result<()> {
        for s in &mut self.systems {
            s.update(&mut self.manager, event)?;
        }

        Ok(())
    }
}
