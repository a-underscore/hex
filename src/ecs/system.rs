use super::{AsAny, World};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub trait System: AsAny {
    fn update(
        &mut self,
        world: &Rc<RefCell<World>>,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()>;
}
