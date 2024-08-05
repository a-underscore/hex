pub mod as_any;

pub use as_any::AsAny;

use crate::Id;
use parking_lot::RwLock;
use std::{any::Any, collections::HashMap, sync::Arc};

pub struct ComponentManager<C: Send + Sync + 'static> {
    pub(crate) components: HashMap<Id, Arc<RwLock<C>>>,
}

impl<C: Send + Sync + 'static> ComponentManager<C> {
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            components: HashMap::new(),
        }))
    }
}

impl<C: Send + Sync + 'static> AsAny for Arc<RwLock<ComponentManager<C>>> {
    fn as_any(&self) -> &(dyn Any + Send + Sync + 'static) {
        self
    }

    fn remove(&self, eid: Id) -> bool {
        self.write().components.remove(&eid);

        self.read().components.is_empty()
    }
}
