use crate::{AsAny, Entity};
use glium::glutin::event::Event;
use std::{rc::Rc, time::Duration};

pub trait Component: AsAny {
    fn id(&self) -> Rc<String>;

    fn tid(&self) -> Rc<String>;

    fn init(self: Rc<Self>, _parent: Option<Rc<Entity>>) {}

    fn update(self: Rc<Self>, _parent: Option<Rc<Entity>>, _event: &Event<()>, _delta: Duration) {}

    fn parent(&self) -> Option<Rc<Entity>>;

    fn set_parent(&self, parent: Option<Rc<Entity>>);
}
