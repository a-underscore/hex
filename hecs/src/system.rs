use crate::{AsAny, Entity, Id};
use std::rc::Rc;

pub trait System: AsAny {
    fn id(&self) -> Id;

    fn update(self: Rc<Self>, entity: Rc<Entity>);
}
