use crate::{as_any::AsAny, entity::Entity};
use std::sync::Arc;

pub trait Component: AsAny {
    fn id(self: Arc<Self>) -> Arc<String>;

    fn tid(self: Arc<Self>) -> Arc<String>;

    fn on_init(self: Arc<Self>, _owner: Option<Arc<Entity>>) {}

    fn on_update(self: Arc<Self>, _owner: Option<Arc<Entity>>) {}

    fn on_remove(self: Arc<Self>, _owner: Option<Arc<Entity>>) {}
}
