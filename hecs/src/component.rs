use crate::{AsAny, Id};
use std::any::Any;

pub trait Component: AsAny {
    fn get_id() -> Id;
}

impl<C> AsAny for C
where
    C: Component + Sized + 'static,
{
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
