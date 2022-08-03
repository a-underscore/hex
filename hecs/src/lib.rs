pub mod as_any;
pub mod component;
pub mod entity;

pub use as_any::AsAny;
pub use component::Component;
pub use entity::{Entity, EntityData, ENTITY_ID};
pub use hecs_derive as derive;

use std::{rc::Rc, thread::LocalKey};

pub fn id(id: &str) -> Rc<String> {
    Rc::new(id.to_string())
}

pub fn tid(id: &'static LocalKey<Rc<String>>) -> Rc<String> {
    id.with(|id| id.clone())
}
