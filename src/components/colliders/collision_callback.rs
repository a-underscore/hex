use crate::ecs::{Component, Entity, Id};
use std::{cell::RefCell, rc::Rc};

pub trait CollisionCallback: 'static + Component {
    fn callback(&mut self, parent: (Id, Rc<RefCell<Entity>>), other: (Id, Rc<RefCell<Entity>>));
}
