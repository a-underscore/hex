use winit::{event::Event, event_loop::EventLoopWindowTarget};

pub struct Control<'a> {
    pub event: Event<()>,
    pub elwt: &'a EventLoopWindowTarget<()>,
}

impl<'a> Control<'a> {
    pub fn new(event: Event<()>, elwt: &'a EventLoopWindowTarget<()>) -> Self {
        Self { event, elwt }
    }
}
