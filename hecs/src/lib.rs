pub mod as_any;
pub mod component;
pub mod entity;

pub use as_any::AsAny;
pub use component::Component;
pub use entity::{Entity, EntityData, ENTITY_ID};
pub use glium;
pub use hecs_derive as derive;

use std::{rc::Rc, thread::LocalKey};

pub type Id = Rc<String>;
pub type Parent = Option<Rc<Entity>>;

pub fn id(id: &str) -> Id {
    Rc::new(id.to_string())
}

pub fn tid(id: &'static LocalKey<Id>) -> Id {
    id.with(|id| id.clone())
}
