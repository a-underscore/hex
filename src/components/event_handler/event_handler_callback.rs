use crate::ecs::{Entity, Id};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc};

pub trait EventHandlerCallback {
    fn callback(&mut self, parent: (Id, Rc<RefCell<Entity>>), event: &Event<()>);
}
