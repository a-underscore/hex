use crate::{
    components::{Camera, Sprite, Transform, CAMERA_ID, SPRITE_ID, TRANSFORM_ID},
    ecs::{self, Id, System, World},
    Engine,
};
use glium::Surface;
use std::{cell::RefCell, rc::Rc};

thread_local! {
    pub static DRAWING_SYSTEM_ID: Id = ecs::id("drawing_system");
}

pub struct DrawingSystem<'a> {
    pub engine: Rc<RefCell<Engine<'a>>>,
}

impl<'a> DrawingSystem<'a> {
    pub fn new(engine: Rc<RefCell<Engine<'a>>>) -> Rc<RefCell<Box<Self>>> {
        Rc::new(RefCell::new(Box::new(Self { engine })))
    }
}

impl System for DrawingSystem<'static> {
    fn id(&self) -> Id {
        ecs::tid(&DRAWING_SYSTEM_ID)
    }

    fn on_update(&mut self, world: &mut World) {
        if let Some((camera, transform)) = world
            .entities
            .values()
            .filter_map(|e| {
                let e = e.borrow();

                e.get::<Camera>(&ecs::tid(&CAMERA_ID)).and_then(|c| {
                    e.get::<Transform>(&ecs::tid(&TRANSFORM_ID))
                        .and_then(|t| Some((c, t)))
                })
            })
            .find(|(c, _)| c.borrow().active)
        {
            let engine = self.engine.borrow();
            let mut frame = engine.display.draw();

            frame.clear_color_and_depth(engine.scene.borrow().bg.into(), 1.0);

            for (s, t) in world
                .entities
                .values()
                .filter_map(|e| {
                    let e = e.borrow();

                    e.get::<Sprite>(&ecs::tid(&SPRITE_ID)).and_then(|s| {
                        e.get::<Transform>(&ecs::tid(&TRANSFORM_ID))
                            .and_then(|t| Some((s, t)))
                    })
                })
                .collect::<Vec<_>>()
            {
                s.borrow()
                    .draw(
                        t.borrow().as_ref(),
                        camera.borrow().as_ref(),
                        transform.borrow().as_ref(),
                        &self.engine.borrow(),
                        &mut frame,
                    )
                    .unwrap();
            }

            frame.finish().unwrap();
        }
    }
}
