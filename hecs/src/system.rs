use crate::{AsAny, Entity, Id};
use glium::glutin::event::Event;
use std::{rc::Rc, time::Duration};

pub trait System: AsAny {
    fn id(&self) -> Id;

    fn on_init(self: Rc<Self>, entity: Rc<Entity>);

    fn on_update(self: Rc<Self>, entity: Rc<Entity>, event: &Event<()>, delta: Duration);
}
