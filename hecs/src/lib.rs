pub mod as_any;
pub mod component;
pub mod entity;

pub use as_any::AsAny;
pub use component::Component;
pub use entity::{Entity, ENTITY_ID};
pub use hecs_derive as derive;

use std::rc::Rc;

pub fn id(id: &str) -> Rc<String> {
    Rc::new(id.to_string())
}
