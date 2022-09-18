use crate::{
    assets::Scene,
    components::{Camera, Sprite, Transform},
    ecs::{self, Component, Id, System, World},
};
use glium::{glutin::event::Event, Display, Surface};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
    time::Duration,
};

pub struct DrawingSystem {
    pub display: Rc<RefCell<Display>>,
    pub scene: Rc<RefCell<Scene>>,
}

impl DrawingSystem {
    thread_local! {
        pub static ID: Id = ecs::id("drawing_system");
    }

    pub fn new(display: Rc<RefCell<Display>>, scene: Rc<RefCell<Scene>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { display, scene }))
    }
}

impl DrawingSystem {
    fn draw_sprites(&self, world: &World) -> anyhow::Result<()> {
        if let Some((ca, ct)) = world
            .get_all_with(&[&Camera::get_id(), &Transform::get_id()])
            .iter()
            .find_map(|(_, c)| match c.as_slice() {
                [(_, ca), (_, ct)] => {
                    let ca = Ref::filter_map(ca.try_borrow().ok()?, |ca| {
                        ca.as_any_ref().downcast_ref::<Camera>()
                    })
                    .ok()?;
                    let ct = Ref::filter_map(ct.try_borrow().ok()?, |ct| {
                        ct.as_any_ref().downcast_ref::<Transform>()
                    })
                    .ok()?;

                    if ca.active {
                        Some((ca, ct))
                    } else {
                        None
                    }
                }
                _ => None,
            })
        {
            let scene = self.scene.try_borrow()?;
            let display = self.display.try_borrow()?;
            let mut target = display.draw();

            target.clear_color_and_depth(scene.bg.into(), 1.0);

            for (_, c) in world.get_all_with(&[&Sprite::get_id(), &Transform::get_id()]) {
                if let [(_, s), (_, t)] = c.as_slice() {
                    if let (Some(s), Some(t)) = (
                        s.try_borrow()?.as_any_ref().downcast_ref::<Sprite>(),
                        t.try_borrow()?.as_any_ref().downcast_ref::<Transform>(),
                    ) {
                        s.draw(&display, &mut target, &t, &ca, &ct)?;
                    }
                }
            }

            target.finish()?;
        }

        Ok(())
    }
}

impl Component for DrawingSystem {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}

impl System for DrawingSystem {
    fn update(&mut self, world: &mut World, _event: &Event<()>, _delta: Duration) {
        if let Err(e) = self.draw_sprites(world) {
            println!("{}", e);
        }
    }
}
