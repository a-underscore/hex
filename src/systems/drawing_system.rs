use crate::{
    components::{Camera, Sprite, Transform},
    ecs::{self, Component, Id, System, World, ToRef},
};
use cgmath::Vector4;
use glium::{glutin::event::Event, Display, Surface};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
    time::Duration,
};

#[derive(Clone)]
pub struct DrawingSystem {
    pub display: Rc<RefCell<Display>>,
    pub bg: Vector4<f32>,
}

impl DrawingSystem {
    thread_local! {
        pub static ID: Id = ecs::id("drawing_system");
    }

    pub fn new(display: &Rc<RefCell<Display>>, bg: Vector4<f32>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            display: display.clone(),
            bg,
        }))
    }
}

impl Component for DrawingSystem {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}

impl System for DrawingSystem {
    fn update(
        &mut self,
        world: &Rc<RefCell<World>>,
        _: &Event<()>,
        _: Duration,
    ) -> anyhow::Result<()> {
        let world = world.try_borrow()?;

        if let Some(((_, e), ca)) = world
            .get_all_ref::<Camera>()
            .into_iter()
            .find(|(_, c)| c.active)
        {
            if let Some(ct) = e.try_borrow()?.get_ref::<Transform>() {
                let display = self.display.try_borrow()?;
                let mut target = display.draw();

                target.clear_color_and_depth(self.bg.into(), 1.0);

                for (_, c) in world.get_all_with(&[&Sprite::get_id(), &Transform::get_id()]) {
                    if let [(_, s), (_, t)] = c.as_slice() {
                        if let (Some(s), Some(t)) = (
                            Ref::filter_map(s.try_borrow()?, |s| s.to_ref::<Sprite>()).ok(),
                            Ref::filter_map(t.try_borrow()?, |t| t.to_ref::<Transform>()).ok(),
                        ) {
                            s.draw(&display, &mut target, &t, &ca, &ct)?;
                        }
                    }
                }

                target.finish()?;
            }
        }

        Ok(())
    }
}
