use crate::{AsAny, Entity, Id, Parent};
use glium::glutin::event::Event;
use std::{rc::Rc, time::Duration};

pub trait Component: AsAny {
    fn id(&self) -> Id;

    fn tid(&self) -> Id;

    fn parent(&self) -> Parent;

    fn on_init(self: Rc<Self>, _parent: Option<Rc<Entity>>) {}

    fn on_update(
        self: Rc<Self>,
        _parent: Option<Rc<Entity>>,
        _event: &Event<()>,
        _delta: Duration,
    ) {
    }
}
