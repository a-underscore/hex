use super::{AsAny, Id};
use std::any::Any;

pub trait Component {
    fn get_id() -> Id;
}

impl<C> AsAny for C
where
    C: Component + 'static,
{
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
