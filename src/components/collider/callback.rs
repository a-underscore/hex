use crate::ecs::{Entity, Id, World};
use cgmath::Vector2;
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub trait Callback {
    fn callback(
        &mut self,
        parent: &(Id, Rc<RefCell<Entity>>),
        other: &(Id, Rc<RefCell<Entity>>),
        world: &mut World,
        position: Vector2<f32>,
        event: &Event<()>,
        delta: Duration,
    );
}
