use std::{any::Any, sync::Arc};

pub trait AsAny: Send + Sync + 'static {
    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync + 'static>;
}
