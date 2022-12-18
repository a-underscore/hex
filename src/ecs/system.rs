use super::Manager;
use glium::glutin::event::Event;

pub trait System<'a>: 'a {
    fn init(&mut self, _: &mut Manager) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(&mut self, _: &mut Manager, _: &Event<()>) -> anyhow::Result<()> {
        Ok(())
    }
}
