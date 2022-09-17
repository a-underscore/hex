use crate::ecs::{Component, System, World};
use cgmath::Vector4;
use glium::{
    glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder, NotCurrent},
    Display,
};
use std::{cell::RefCell, rc::Rc};

pub struct Scene {
    pub bg: Vector4<f32>,
    pub world: Rc<RefCell<World>>,
}

impl Scene {
    pub fn new(bg: Vector4<f32>, world: Rc<RefCell<World>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { bg, world }))
    }

    pub fn add_system<S>(&self, system: Rc<RefCell<S>>) -> anyhow::Result<()>
    where
        S: System + Component + 'static,
    {
        self.world.try_borrow_mut()?.add_system(&system);

        Ok(())
    }

    pub fn setup_display(
        wb: WindowBuilder,
        cb: ContextBuilder<'_, NotCurrent>,
    ) -> anyhow::Result<(EventLoop<()>, Rc<RefCell<Display>>)> {
        let event_loop = EventLoop::new();
        let display = Rc::new(RefCell::new(Display::new(wb, cb, &event_loop)?));

        Ok((event_loop, display))
    }

    pub fn basic_display<S>(
        name: S,
        sample_count: u16,
    ) -> anyhow::Result<(EventLoop<()>, Rc<RefCell<Display>>)>
    where
        S: Into<String>,
    {
        let wb = WindowBuilder::new().with_title(name);
        let cb = ContextBuilder::new().with_multisampling(sample_count);

        Self::setup_display(wb, cb)
    }
}
