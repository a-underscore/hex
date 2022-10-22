use crate::ecs::{GenericEntity, World};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub trait EventHandlerCallback {
    fn callback(
        &mut self,
        parent: &GenericEntity,
        world: &Rc<RefCell<World>>,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()>;
}
