pub mod as_any;
pub mod component;
pub mod entity;
pub mod system;
pub mod world;

pub use as_any::AsAny;
pub use component::Component;
pub use entity::Entity;
pub use system::System;
pub use world::World;

use std::{rc::Rc, thread::LocalKey};

pub type Id = Rc<String>;

pub fn id(id: &str) -> Id {
    Rc::new(id.to_string())
}

pub fn tid(id: &'static LocalKey<Id>) -> Id {
    id.with(|c| c.clone())
}
