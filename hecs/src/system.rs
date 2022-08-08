use crate::{AsAny, Id, World};
use glium::glutin::event::Event;
use std::time::Duration;

pub trait System: AsAny + 'static {
    fn id(&self) -> Id;

    fn on_init(&mut self, _world: &mut World) {}

    fn on_update(&mut self, _world: &mut World, _event: &Event<()>, _delta: Duration) {}
}
