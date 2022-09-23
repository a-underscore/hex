use super::{AsAny, World};
use glium::glutin::event::Event;
use std::time::Duration;

pub trait System: AsAny {
    fn update(
        &mut self,
        world: &mut World,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()>;
}
