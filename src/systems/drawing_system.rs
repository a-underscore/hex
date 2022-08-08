use crate::{
    components::{Camera, Sprite, Transform, CAMERA_ID, SPRITE_ID, TRANSFORM_ID},
    ecs::{self, Id, System, World},
    Engine,
};
use glium::{glutin::event::Event, Surface};
use std::{cell::RefCell, rc::Rc, time::Duration};

thread_local! {
    pub static DRAWING_SYSTEM_ID: Id = ecs::id("drawing_system");
}

pub struct DrawingSystem<'a> {
    pub engine: Rc<Engine<'a>>,
}

impl<'a> DrawingSystem<'a> {
    pub fn new(engine: Rc<Engine<'a>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { engine }))
    }
}

impl<'a> DrawingSystem<'a> {
    pub fn draw_sprites(&self, world: &World) -> anyhow::Result<()> {
        for (camera, transform) in world.entities().values().filter_map(|e| {
            let e = e.borrow();

            e.get(&ecs::tid(&CAMERA_ID))
                .and_then(|c| e.get(&ecs::tid(&TRANSFORM_ID)).and_then(|t| Some((c, t))))
        }) {
            if let (Some(camera), Some(transform)) = (
                (*camera.borrow()).as_any_ref().downcast_ref::<Camera>(),
                (*transform.borrow())
                    .as_any_ref()
                    .downcast_ref::<Transform>(),
            ) {
                if camera.get_active() {
                    let mut frame = self.engine.display.draw();

                    frame.clear_color_and_depth(self.engine.scene.borrow().bg.into(), 1.0);

                    for (s, t) in world.entities().values().filter_map(|e| {
                        let e = e.borrow();

                        e.get(&ecs::tid(&SPRITE_ID)).and_then(|s| {
                            e.get(&ecs::tid(&TRANSFORM_ID)).and_then(|t| Some((s, t)))
                        })
                    }) {
                        if let (Some(s), Some(t)) = (
                            (*s.borrow()).as_any_ref().downcast_ref::<Sprite>(),
                            (*t.borrow()).as_any_ref().downcast_ref::<Transform>(),
                        ) {
                            s.draw(&t, &camera, &transform, &self.engine, &mut frame)?;
                        }
                    }

                    frame.finish().unwrap();
                }
            }
        }

        Ok(())
    }
}

impl System for DrawingSystem<'static> {
    fn id(&self) -> Id {
        ecs::tid(&DRAWING_SYSTEM_ID)
    }

    fn on_update(&mut self, world: &mut World, _event: &Event<()>, _delta: Duration) {
        self.draw_sprites(world).unwrap();
    }
}
