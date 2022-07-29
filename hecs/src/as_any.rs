use std::{any::Any, rc::Rc};

pub trait AsAny: 'static {
    fn as_any(self: Rc<Self>) -> Rc<dyn Any>;
}
