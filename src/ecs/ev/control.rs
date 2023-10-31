use winit::{event::Event, event_loop::ControlFlow};

pub struct Control {
    pub event: Event<()>,
    pub flow: Option<ControlFlow>,
}

impl Control {
    pub fn new(event: Event<()>) -> Self {
        Self { event, flow: None }
    }
}
