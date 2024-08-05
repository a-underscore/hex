use crate::Id;
use std::any::Any;

pub trait AsAny: Send + Sync + 'static {
    fn as_any(&self) -> &(dyn Any + Send + Sync + 'static);

    fn remove(&self, eid: Id) -> bool;
}
