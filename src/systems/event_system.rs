use crate::ecs::{Manager, System};
use glium::glutin::event::Event;

#[derive(Default)]
pub struct EventSystem;

impl System for EventSystem {
    fn update(&mut self, manager: &mut Manager, event: &Event<()>) -> anyhow::Result<()> {
        unimplemented!()
    }
}
