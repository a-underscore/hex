use super::Manager;
use glium::glutin::event::Event;

pub trait System<'a>: 'a {
    fn update(&mut self, manager: &mut Manager, event: &Event<()>) -> anyhow::Result<()>;
}
