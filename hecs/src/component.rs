use crate::{AsAny, Id, Parent};
use glium::glutin::event::Event;
use std::{rc::Rc, time::Duration};

pub trait Component: AsAny {
    fn id(&self) -> Id;

    fn tid(&self) -> Id;

    fn get_parent(&self) -> Parent;

    fn set_parent(&self, parent: Parent);

    fn on_init(self: Rc<Self>, _parent: Parent) {}

    fn on_update(self: Rc<Self>, _parent: Parent, _event: &Event<()>, _delta: Duration) {}
}
