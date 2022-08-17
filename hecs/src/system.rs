use crate::{AsAny, Id, World};
use glium::glutin::event::Event;
use std::time::Duration;

pub trait System: AsAny + 'static {
    fn id(&self) -> Id;

    fn init(&mut self, _world: &mut World) {}

    fn update(&mut self, _world: &mut World, _event: &Event<()>, _delta: Duration) {}
}
