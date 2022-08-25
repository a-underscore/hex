use crate::{
    assets::Engine,
    components::{Camera, Sprite, Transform},
    ecs::{self, Component, Id, System, World},
};
use glium::{glutin::event::Event, Surface};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
    time::Duration,
};

pub struct DrawingSystem<'a> {
    pub engine: Rc<RefCell<Engine<'a>>>,
}

impl<'a> DrawingSystem<'a> {
    thread_local! {
        pub static ID: Id = ecs::id("drawing_system");
    }

    pub fn new(engine: Rc<RefCell<Engine<'a>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { engine }))
    }
}

impl<'a> DrawingSystem<'a> {
    fn draw_sprites(&self, world: &World) -> anyhow::Result<()> {
        if let Some((ca, ct)) = world
            .get_all_with(&[&Camera::get_id(), &Transform::get_id()])
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
            let mut frame = self.engine.borrow().display.draw();

            frame.clear_color_and_depth(self.engine.borrow().scene.borrow().bg.into(), 1.0);

            for (_, c) in world.get_all_with(&[&Sprite::get_id(), &Transform::get_id()]) {
                if let [(_, s), (_, t)] = c.as_slice() {
                    if let (Some(s), Some(t)) = (
                        s.borrow().as_any_ref().downcast_ref::<Sprite>(),
                        t.borrow().as_any_ref().downcast_ref::<Transform>(),
                    ) {
                        s.draw(&t, &ca, &ct, &self.engine.borrow(), &mut frame)?
                    }
                }
            }

            frame.finish().unwrap();
        }

        Ok(())
    }
}

impl Component for DrawingSystem<'static> {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}

impl System for DrawingSystem<'static> {
    fn update(&mut self, world: &mut World, _event: &Event<()>, _delta: Duration) {
        self.draw_sprites(world).unwrap();
    }
}
