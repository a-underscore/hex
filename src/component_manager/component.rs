use super::AsAny;
use parking_lot::RwLock;
use std::{any::Any, sync::Arc};

pub trait Component: Send + Sync + 'static {}

impl<C: Component> AsAny for Arc<RwLock<C>> {
    fn as_any(&self) -> &(dyn Any + Send + Sync + 'static) {
        self
    }
}
