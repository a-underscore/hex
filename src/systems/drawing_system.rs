use crate::{
    components::{Camera, Sprite, Transform},
    ecs::{self, Component, Id, System, Type, World},
};
use cgmath::Vector4;
use glium::{glutin::event::Event, Display, Surface};

#[derive(Clone)]
pub struct DrawingSystem {
    pub display: Type<Display>,
    pub bg: Vector4<f32>,
}

impl DrawingSystem {
    pub fn new(display: Type<Display>, bg: Vector4<f32>) -> Type<Self> {
        ecs::new(Self { display, bg })
    }
}

impl Component for DrawingSystem {
    fn id() -> Id {
        ecs::id("drawing_system")
    }
}

impl System for DrawingSystem {
    fn update(&mut self, world: &mut World, event: &Event<()>) -> anyhow::Result<()> {
        if let Event::MainEventsCleared = event {
            if let Some(((_, e), ca)) = world.entities().iter().find_map(|p @ (_, e)| {
                e.try_borrow()
                    .ok()?
                    .get::<Camera>()
                    .and_then(|c| c.try_borrow().ok()?.active.then_some((p, c.clone())))
            }) {
                if let Some(ct) = e.try_borrow()?.get::<Transform>() {
                    let display = self.display.try_borrow()?;
                    let mut target = display.draw();

                    target.clear_color_and_depth(self.bg.into(), 1.0);

                    world
                        .entities()
                        .iter()
                        .filter_map(|(_, e)| {
                            e.try_borrow()
                                .ok()
                                .and_then(|e| Some((e.get::<Sprite>()?, e.get::<Transform>()?)))
                        })
                        .try_for_each(|(s, t)| {
                            s.try_borrow()?.draw(
                                &mut target,
                                &*t.try_borrow()?,
                                &*ca.try_borrow()?,
                                &*ct.try_borrow()?,
                            )
                        })?;

                    target.finish()?;
                }
            }
        }

        Ok(())
    }
}
