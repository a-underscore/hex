use super::AsAny;
use std::any::Any;

pub trait Component: Send + Sync + 'static {}

impl<C> AsAny for C
where
    C: Component,
{
    fn as_any(&self) -> &(dyn Any + Send + Sync + 'static) {
        self
    }

    fn as_any_mut(&mut self) -> &mut (dyn Any + Send + Sync + 'static) {
        self
    }
}
