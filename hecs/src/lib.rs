pub mod as_any;
pub mod component;
pub mod entity;

pub use as_any::AsAny;
pub use component::Component;
pub use entity::{Entity, ENTITY_ID};
pub use hecs_derive as derive;

use std::{cell::RefCell, rc::Rc, thread::LocalKey};

pub fn id(id: &str) -> Rc<String> {
    Rc::new(id.to_string())
}

pub fn tid(id: &'static LocalKey<Rc<String>>) -> Rc<String> {
    id.with(|id| id.clone())
}

pub fn parent(parent: &Option<Rc<Entity>>) -> Rc<RefCell<Option<Rc<Entity>>>> {
    Rc::new(RefCell::new(parent.clone()))
}
