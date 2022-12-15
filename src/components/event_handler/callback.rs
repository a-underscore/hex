use crate::ecs::Manager;
use glium::glutin::event::Event;

pub trait Callback<'a>: 'a {
    fn callback(
        &mut self,
        id: usize,
        manager: &mut Manager,
        event: &Event<()>,
    ) -> anyhow::Result<()>;
}
