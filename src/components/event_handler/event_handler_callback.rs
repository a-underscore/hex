use super::EventHandler;
use crate::{
    ecs::{Entity, Id, World},
    systems::EventSystem,
};
use glium::glutin::event::Event;

pub trait EventHandlerCallback {
    fn callback(
        &mut self,
        event_handler: &mut EventHandler,
        parent: &(Id, &mut Entity),
        system: &mut EventSystem,
        world: &mut World,
        event: &Event<()>,
    ) -> anyhow::Result<()>;
}
