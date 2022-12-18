use crate::{
    components::{Camera, Sprite, Transform},
    ecs::{Manager, System},
};
use cgmath::Vector4;
use glium::{glutin::event::Event, Display, Surface};

pub struct DrawingSystem {
    pub display: Display,
    pub bg: Vector4<f32>,
}

impl DrawingSystem {
    pub fn new(display: Display, bg: Vector4<f32>) -> Self {
        Self { display, bg }
    }
}

impl<'a> System<'a> for DrawingSystem {
    fn update(&mut self, manager: &mut Manager<'a>, event: &Event<()>) -> anyhow::Result<()> {
        if let Event::MainEventsCleared = event {
            if let Some((c, ct)) = manager.entities().into_iter().find_map(|e| {
                manager
                    .get_c::<Camera>(e)
                    .and_then(|c| c.active.then_some(c))
                    .and_then(|c| Some((c, manager.get_c::<Transform>(e)?)))
            }) {
                let mut target = self.display.draw();

                target.clear_color_and_depth(self.bg.into(), 1.0);

                for e in manager.entities() {
                    if let Some((s, t)) = manager.get_c::<Sprite>(e).and_then(|s| {
                        Some((s.active.then_some(s)?, manager.get_c::<Transform>(e)?))
                    }) {
                        s.draw(&mut target, t, c, ct)?;
                    }
                }
            }
        }

        Ok(())
    }
}
