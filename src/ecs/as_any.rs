use super::{Component, ToMut, ToRef};
use std::any::Any;

pub trait AsAny {
    fn as_any_ref(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl ToRef for dyn AsAny {
    fn to_ref<C>(&self) -> Option<&C>
    where
        C: Component + 'static,
    {
        self.as_any_ref().downcast_ref()
    }
}

impl ToMut for dyn AsAny {
    fn to_mut<C>(&mut self) -> Option<&mut C>
    where
        C: Component + 'static,
    {
        self.as_any_mut().downcast_mut()
    }
}
