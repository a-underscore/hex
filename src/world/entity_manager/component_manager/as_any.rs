use crate::Id;
use std::any::Any;

pub trait AsAny: Send + Sync + 'static {
    fn as_any(&self) -> &(dyn Any + Send + Sync + 'static);

    fn as_any_mut(&mut self) -> &mut (dyn Any + Send + Sync + 'static);

    fn remove(&mut self, eid: Id) -> bool;
}
