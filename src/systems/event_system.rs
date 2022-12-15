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
            if let Some(mut ev) = manager.rm_c::<EventHandler>(e) {
                if ev.active {
                    ev.active = ev.callback.callback(e, manager, event)?;
                }

                manager.add_c(e, ev);
            }
        }

        Ok(())
    }
}
