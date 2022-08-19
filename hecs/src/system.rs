use crate::{AsAny, World};
use glium::glutin::event::Event;
use std::time::Duration;

pub trait System: AsAny {
    fn init(&mut self, _world: &mut World) {}

    fn update(&mut self, _world: &mut World, _event: &Event<()>, _delta: Duration) {}
}
