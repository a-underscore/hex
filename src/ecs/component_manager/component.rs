use super::AsAny;
use std::any::Any;

pub trait Component: 'static {}

impl<C> AsAny for C
where
    C: Component + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
