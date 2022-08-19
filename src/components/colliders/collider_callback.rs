use crate::ecs::{Entity, Id};
use cgmath::Vector2;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub trait ColliderCallback: 'static {
    fn callback(
        &mut self,
        parent: (Id, Rc<RefCell<Entity>>),
        other: (Id, Rc<RefCell<Entity>>),
        points: &[Vector2<f32>],
        delta: Duration,
    );
}
