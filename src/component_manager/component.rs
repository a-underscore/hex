use super::AsAny;
use std::{
    any::Any,
    sync::{Arc, RwLock},
};

pub trait Component: Send + Sync + 'static {}

impl<C> AsAny for Arc<RwLock<C>>
where
    C: Component,
{
    fn as_any(&self) -> &(dyn Any + Send + Sync + 'static) {
        self
    }
}
