use std::{any::Any, rc::Rc};

pub trait AsAny {
    fn as_any(self: Rc<Self>) -> Rc<dyn Any>;

    fn as_any_ref(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> AsAny for T
where
    T: Sized + 'static,
{
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
