use glium::glutin::{event::Event, event_loop::ControlFlow};

pub struct Control<'a, 'b> {
    pub event: Event<'a, ()>,
    pub flow: &'b mut ControlFlow,
}

impl<'a, 'b> Control<'a, 'b> {
    pub fn new(event: Event<'a, ()>, flow: &'b mut ControlFlow) -> Self {
        Self { event, flow }
    }
}
