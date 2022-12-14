use super::Manager;
use glium::glutin::event::Event;

pub trait System {
    fn update(&mut self, manager: &mut Manager, event: &Event<()>) -> anyhow::Result<()>;
}
