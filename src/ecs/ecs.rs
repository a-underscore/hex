use super::World;
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
        .and_then(|w| Ok(w.clone()))?
        .get_systems()
        .values()
    {
        s.try_borrow_mut()?.update(world, event, delta)?;
    }

    Ok(())
}
