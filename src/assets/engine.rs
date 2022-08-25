use crate::assets::Scene;
use glium::{
    draw_parameters::{Blend, DepthTest},
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder, NotCurrent,
    },
    Depth, Display, DrawParameters,
};
use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

pub struct Engine<'a> {
    pub display: Display,
    pub scene: Rc<RefCell<Scene>>,
    pub draw_parameters: DrawParameters<'a>,
}

impl<'a> Engine<'a> {
    pub fn new(
        display: Display,
        scene: Rc<RefCell<Scene>>,
        draw_parameters: DrawParameters<'a>,
    ) -> anyhow::Result<Rc<RefCell<Self>>> {
        Ok(Rc::new(RefCell::new(Self {
            display,
            scene,
            draw_parameters,
        })))
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

    pub fn default_draw_parameters() -> DrawParameters<'static> {
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

pub fn init_scene(engine: Rc<RefCell<Engine>>) {
    let world = engine.borrow().scene.borrow().world.clone();

    world.borrow_mut().init_systems();
}

pub fn update_scene(engine: Rc<RefCell<Engine>>, event: &Event<()>, delta: Duration) {
    let world = engine.borrow().scene.borrow().world.clone();

    world.borrow_mut().update_systems(&event, delta);
}

pub fn init(engine: Rc<RefCell<Engine<'static>>>, event_loop: EventLoop<()>) {
    init_scene(engine.clone());

    let mut old_frame_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let frame_time = Instant::now();
        let delta = frame_time.duration_since(old_frame_time);

        old_frame_time = frame_time;

        update_scene(engine.clone(), &event, delta);

        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => {}
        }
    });
}
