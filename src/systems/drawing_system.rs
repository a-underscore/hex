use crate::{
    components::{Camera, Sprite, Transform},
    ecs::{self, Id, System, World},
    Engine,
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
        if let Some((ca, ct)) = world
            .get_all_with(&[&ecs::tid(&Camera::ID), &ecs::tid(&Transform::ID)])
            .iter()
            .find_map(|(_, c)| match c.as_slice() {
                [(_, ca), (_, ct)] => {
                    let ca =
                        Ref::filter_map(ca.borrow(), |ca| ca.as_any_ref().downcast_ref::<Camera>())
                            .ok()?;
                    let ct = Ref::filter_map(ct.borrow(), |ct| {
                        ct.as_any_ref().downcast_ref::<Transform>()
                    })
                    .ok()?;

                    if ca.get_active() {
                        Some((ca, ct))
                    } else {
                        None
                    }
                }
                _ => None,
            })
        {
            let mut frame = self.engine.display.draw();

            frame.clear_color_and_depth(self.engine.scene.borrow().bg.into(), 1.0);

            for (_, c) in world.get_all_with(&[&ecs::tid(&Sprite::ID), &ecs::tid(&Transform::ID)]) {
                if let [(_, s), (_, t)] = c.as_slice() {
                    if let (Some(s), Some(t)) = (
                        s.borrow().as_any_ref().downcast_ref::<Sprite>(),
                        t.borrow().as_any_ref().downcast_ref::<Transform>(),
                    ) {
                        s.draw(&t, &ca, &ct, &self.engine, &mut frame)?
                    }
                }
            }

            frame.finish().unwrap();
        }

        Ok(())
    }
}

impl System for DrawingSystem<'static> {
    fn id(&self) -> Id {
        ecs::tid(&DRAWING_SYSTEM_ID)
    }

    fn update(&mut self, world: &mut World, _event: &Event<()>, _delta: Duration) {
        self.draw_sprites(world).unwrap();
    }
}
