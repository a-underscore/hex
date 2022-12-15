use super::{Manager, System};
use glium::glutin::event::Event;

#[derive(Default)]
pub struct World<'a> {
    pub manager: Manager<'a>,
    pub systems: Vec<Box<dyn System>>,
}

impl<'a> World<'a> {
    pub fn add_s<S>(&mut self, s: S)
    where
        S: System + 'static,
    {
        self.systems.push(Box::new(s));
    }

    pub fn update(&mut self, event: &Event<()>) -> anyhow::Result<()> {
        for s in &mut self.systems {
            s.update(&mut self.manager, event)?;
        }

        Ok(())
    }
}
