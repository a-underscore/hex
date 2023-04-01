use glium::glutin::{event::Event, event_loop::ControlFlow};

pub struct Control<'a> {
    pub event: Event<'a, ()>,
    pub flow: Option<ControlFlow>,
}

impl<'a> Control<'a> {
    pub fn new(event: Event<'a, ()>) -> Self {
        Self { event, flow: None }
    }
}
