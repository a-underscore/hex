use crate::{
    components::{Camera, Sprite, Transform},
    ecs::{ComponentManager, EntityManager, System},
};
use glium::{glutin::event::Event, Display, Surface};

#[derive(Default)]
pub struct DrawingSystem;

impl<'a> System<'a> for DrawingSystem {
    fn update(
        &mut self,
        entity_manager: &mut EntityManager,
        component_manager: &mut ComponentManager,
        display: &Display,
        event: &Event<()>,
    ) -> anyhow::Result<()> {
        if let Event::MainEventsCleared = event {
            if let Some((c, ct)) = entity_manager.entities.keys().find_map(|e| {
                component_manager
                    .get::<Camera>(entity_manager, *e)
                    .and_then(|c| c.active.then_some(c))
                    .and_then(|c| {
                        Some((c, component_manager.get::<Transform>(entity_manager, *e)?))
                    })
            }) {
                let mut target = display.draw();

                target.clear_color_and_depth(c.bg.into(), 1.0);

                for e in entity_manager.entities.keys() {
                    if let Some((s, t)) = component_manager
                        .get::<Sprite>(entity_manager, *e)
                        .and_then(|s| {
                            Some((
                                s.active.then_some(s)?,
                                component_manager.get::<Transform>(entity_manager, *e)?,
                            ))
                        })
                    {
                        s.draw(&mut target, t, c, ct)?;
                    }
                }

                target.finish()?;
            }
        }

        Ok(())
    }
}
