use crate::ecs::{Component, System, World};
use cgmath::Vector4;
use glium::{
    draw_parameters::{Blend, DepthTest},
    glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder, NotCurrent},
    Depth, Display, DrawParameters,
};
use std::{cell::RefCell, rc::Rc};

pub struct Scene<'a> {
    pub bg: Vector4<f32>,
    pub world: Rc<RefCell<World>>,
    pub draw_params: DrawParameters<'a>,
    pub display: Display,
}

impl<'a> Scene<'a> {
    pub fn new(
        bg: Vector4<f32>,
        world: Rc<RefCell<World>>,
        draw_params: DrawParameters<'a>,
        display: Display,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            bg,
            world,
            draw_params,
            display,
        }))
    }

    pub fn add_system<S>(&self, system: Rc<RefCell<S>>)
    where
        S: System + Component + 'static,
    {
        self.world.borrow_mut().add_system(&system);
    }

    pub fn display(
        wb: WindowBuilder,
        cb: ContextBuilder<'_, NotCurrent>,
    ) -> anyhow::Result<(EventLoop<()>, Display)> {
        let event_loop = EventLoop::new();
        let display = Display::new(wb, cb, &event_loop)?;

        Ok((event_loop, display))
    }

    pub fn basic_display(
        name: &String,
        sample_count: u16,
    ) -> anyhow::Result<(EventLoop<()>, Display)> {
        let wb = WindowBuilder::new().with_title(name);
        let cb = ContextBuilder::new().with_multisampling(sample_count);

        Self::display(wb, cb)
    }

    pub fn default_draw_params() -> DrawParameters<'static> {
        DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: Blend::alpha_blending(),
            ..Default::default()
        }
    }
}
