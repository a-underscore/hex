use crate::{
    components::{Camera, Sprite, Transform, CAMERA_ID, SPRITE_ID, TRANSFORM_ID},
    ecs::{self, Id, System, World},
    Engine, Error,
};
use glium::{glutin::event::Event, Surface};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
    time::Duration,
};

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
    fn draw_sprites(&self, world: &World) -> anyhow::Result<()> {
        world
            .get_entities()
            .values()
            .filter_map(|e| {
                e.borrow()
                    .get_all(&[&ecs::tid(&CAMERA_ID), &ecs::tid(&TRANSFORM_ID)])
                    .and_then(|c| match c.as_slice() {
                        [ca, ct] => Some((ca.clone(), ct.clone())),
                        _ => None,
                    })
            })
            .try_for_each(|(ca, ct)| {
                let ca =
                    Ref::filter_map(ca.borrow(), |ca| ca.as_any_ref().downcast_ref::<Camera>())
                        .map_err(|_| Error::DowncastRefFailed)?;
                let ct = Ref::filter_map(ct.borrow(), |ct| {
                    ct.as_any_ref().downcast_ref::<Transform>()
                })
                .map_err(|_| Error::DowncastRefFailed)?;

                if ca.get_active() {
                    let mut frame = self.engine.display.draw();

                    frame.clear_color_and_depth(self.engine.scene.borrow().bg.into(), 1.0);

                    world
                        .get_entities()
                        .values()
                        .filter_map(|e| {
                            e.borrow()
                                .get_all(&[&ecs::tid(&SPRITE_ID), &ecs::tid(&TRANSFORM_ID)])
                                .and_then(|c| match c.as_slice() {
                                    [s, t] => Some((s.clone(), t.clone())),
                                    _ => None,
                                })
                        })
                        .try_for_each(|(s, t)| {
                            let s = Ref::filter_map(s.borrow(), |s| {
                                s.as_any_ref().downcast_ref::<Sprite>()
                            })
                            .map_err(|_| Error::DowncastRefFailed)?;
                            let t = Ref::filter_map(t.borrow(), |t| {
                                t.as_any_ref().downcast_ref::<Transform>()
                            })
                            .map_err(|_| Error::DowncastRefFailed)?;

                            s.draw(&t, &ca, &ct, &self.engine, &mut frame)
                        })?;

                    frame.finish().unwrap();
                }

                Ok(())
            })
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
