use crate::ecs::Manager;
use glium::glutin::event::Event;

pub trait Callback: Fn(usize, &mut Manager, &Event<()>) -> anyhow::Result<()> {
    fn update(&self, id: usize, manager: &mut Manager, event: &Event<()>) -> anyhow::Result<()> {
        (self)(id, manager, event)
    }
}
