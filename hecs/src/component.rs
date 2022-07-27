use crate::{AsAny, Entity};
use std::rc::Rc;

pub trait Component: AsAny {
    fn id(&self) -> Rc<String>;

    fn tid(&self) -> Rc<String>;

    fn on_update(&self, _owner: Option<&Entity>) {}

    fn on_init(&self, _owner: Option<&Entity>) {}

    fn on_remove(&self, _owner: Option<&Entity>) {}
}
