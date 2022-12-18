use super::Manager;
use glium::glutin::event::Event;

pub trait System<'a>: 'a {
    fn update(&mut self, _: &mut Manager, _: &Event<()>) -> anyhow::Result<()> {
        Ok(())
    }
}
