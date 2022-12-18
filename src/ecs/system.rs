use super::Manager;
use glium::{glutin::event::Event, Display};

pub trait System<'a>: 'a {
    fn init(&mut self, _: &mut Manager, _: &Display) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(&mut self, _: &mut Manager, _: &Display, _: &Event<()>) -> anyhow::Result<()> {
        Ok(())
    }
}
