use std::{any::Any, cell::RefCell, rc::Rc};

pub trait AsAny: 'static {
    fn as_any(self: Rc<Self>) -> Rc<dyn Any>;
}

impl<T> AsAny for RefCell<T>
where
    T: 'static,
{
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self as Rc<dyn Any>
    }
}
