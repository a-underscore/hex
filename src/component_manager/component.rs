use super::AsAny;
use parking_lot::RwLock;
use std::{any::Any, sync::Arc};

pub trait Component: Send + Sync + 'static {}

impl<C> AsAny for Arc<RwLock<C>>
where
    C: Component,
{
    fn as_any(&self) -> &(dyn Any + Send + Sync + 'static) {
        self
    }
}
