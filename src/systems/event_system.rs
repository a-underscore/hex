use crate::{
    components::event_handler::EventHandler,
    ecs::{self, Component, Id, System, Type, World},
};
use glium::glutin::event::Event;

#[derive(Clone)]
pub struct EventSystem;

impl EventSystem {
    pub fn new() -> Type<Self> {
        ecs::new(Self)
    }
}

impl Component for EventSystem {
    fn id() -> Id {
        ecs::id("event_system")
    }
}

impl System for EventSystem {
    fn update(&mut self, world: &mut World, event: &Event<()>) -> anyhow::Result<()> {
        world
            .entities()
            .iter()
            .filter_map(|p @ (_, e)| Some((p, e.try_borrow().ok()?.get::<EventHandler>()?)))
            .try_for_each(|((id, e), c)| {
                c.try_borrow_mut()?.update(
                    &(id.clone(), &mut *e.try_borrow_mut()?),
                    self,
                    world,
                    event,
                )?;

                Ok(())
            })
    }
}
