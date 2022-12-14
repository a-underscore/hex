use crate::{
    components::EventHandler,
    ecs::{Manager, System},
};
use glium::glutin::event::Event;

#[derive(Default)]
pub struct EventSystem;

impl System for EventSystem {
    fn update(&mut self, manager: &mut Manager, event: &Event<()>) -> anyhow::Result<()> {
        for e in manager.entities() {
            if let Some(ev) = manager.get_c::<EventHandler>(e) {
                (ev.callback.clone())(e, manager, event);
            }
        }

        Ok(())
    }
}
