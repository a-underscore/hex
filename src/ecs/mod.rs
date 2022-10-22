pub mod as_any;
pub mod component;
pub mod entity;
pub mod system;
pub mod to_mut;
pub mod to_ref;
pub mod world;

pub use as_any::AsAny;
pub use component::{Component, GenericComponent};
pub use entity::{Entity, GenericEntity};
pub use system::{GenericSystem, System};
pub use to_mut::ToMut;
pub use to_ref::ToRef;
pub use world::World;

use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, thread::LocalKey, time::Duration};

pub type Id = Rc<String>;

pub fn id(id: &str) -> Id {
    Rc::new(id.to_string())
}

pub fn tid(id: &'static LocalKey<Id>) -> Id {
    id.with(|c| c.clone())
}

pub fn update(
    world: &Rc<RefCell<World>>,
    event: &Event<()>,
    delta: Duration,
) -> anyhow::Result<()> {
    for (_, s) in world
        .try_borrow()
        .map(|w| w.clone())?
        .get_systems()
        .values()
    {
        s.try_borrow_mut()?.update(world, event, delta)?;
    }

    Ok(())
}
