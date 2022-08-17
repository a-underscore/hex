use crate::ecs::{Component, Entity, Id};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc};

pub trait EventHandlerCallback: 'static + Component {
    fn callback(&mut self, parent: (Id, Rc<RefCell<Entity>>), event: &Event<()>);
}
