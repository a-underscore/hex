use crate::{AsAny, Entity};
use std::rc::Rc;

pub trait Component: AsAny {
    fn id(&self) -> Rc<String>;

    fn tid(&self) -> Rc<String>;

    fn update(self: Rc<Self>, _parent: Option<Rc<Entity>>) {}

    fn init(self: Rc<Self>, _parent: Option<Rc<Entity>>) {}

    fn parent(&self) -> Option<Rc<Entity>>;

    fn set_parent(&self, parent: Option<Rc<Entity>>);
}
