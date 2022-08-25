use crate::ecs::{Entity, Id, World};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub trait ColliderCallback: 'static {
    fn callback(
        &mut self,
        world: &mut World,
        parent: (Id, Rc<RefCell<Entity>>),
        other: (Id, Rc<RefCell<Entity>>),
        event: &Event<()>,
        delta: Duration,
    );
}
