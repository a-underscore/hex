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

use glium::glutin::event::Event;
use std::{cell::RefCell, mem, rc::Rc};

pub type Id = String;
pub type Type<T> = Rc<RefCell<T>>;

pub fn new<T>(t: T) -> Type<T> {
    Rc::new(RefCell::new(t))
}

pub fn id(id: &str) -> Id {
    id.to_string()
}

pub fn cast<F, T>(f: &Type<F>) -> Type<T>
where
    F: ?Sized,
{
    unsafe { mem::transmute::<_, &Type<T>>(f) }.clone()
}

pub fn update(world: &Type<World>, event: &Event<()>) -> anyhow::Result<()> {
    world.try_borrow_mut()?.update(event)
}
