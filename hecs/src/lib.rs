pub mod as_any;
pub mod component;
pub mod entity;

pub use component::Component;
pub use entity::{Entity, ENTITY_ID};

use std::sync::Arc;

pub fn id(id: &str) -> Arc<String> {
    Arc::new(id.to_string())
}
