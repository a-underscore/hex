use super::World;
use glium::glutin::event::Event;

pub trait System {
    fn update(&mut self, world: &mut World, event: &Event<()>) -> anyhow::Result<()>;
}
