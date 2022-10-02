use crate::ecs::{Entity, Id, World};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub trait Handler {
    fn callback(
        &mut self,
        parent: &(Id, Rc<RefCell<Entity>>),
        world: &Rc<RefCell<World>>,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()>;
}
