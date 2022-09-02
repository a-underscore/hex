use crate::ecs::{Entity, Id, World};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub trait EventHandlerCallback: 'static {
    fn callback(
        &mut self,
        parent: &(Id, Rc<RefCell<Entity>>),
        world: &mut World,
        event: &Event<()>,
        delta: Duration,
    );
}
