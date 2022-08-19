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
use std::{cell::RefCell, rc::Rc, time::Instant};

pub struct Engine<'a> {
    pub display: Display,
    pub scene: Rc<RefCell<Scene>>,
    pub draw_parameters: Rc<RefCell<DrawParameters<'a>>>,
}

impl<'a> Engine<'a> {
    pub fn new(
        display: Display,
        scene: Rc<RefCell<Scene>>,
        draw_parameters: Rc<RefCell<DrawParameters<'a>>>,
    ) -> anyhow::Result<Rc<Self>> {
        Ok(Rc::new(Self {
            display,
            scene,
            draw_parameters,
        }))
    }

    pub fn display(
        wb: WindowBuilder,
        cb: ContextBuilder<'_, NotCurrent>,
    ) -> anyhow::Result<(EventLoop<()>, Display)> {
        let event_loop = EventLoop::new();
        let display = Display::new(wb, cb, &event_loop)?;

        Ok((event_loop, display))
    }

    pub fn default_display() -> anyhow::Result<(EventLoop<()>, Display)> {
        let cb = ContextBuilder::new();

        Self::display(Default::default(), cb)
    }

    pub fn basic_display(
        name: &String,
        sample_count: u16,
    ) -> anyhow::Result<(EventLoop<()>, Display)> {
        let wb = WindowBuilder::new().with_title(name);
        let cb = ContextBuilder::new().with_multisampling(sample_count);

        Self::display(wb, cb)
    }

    pub fn default_draw_parameters() -> Rc<RefCell<DrawParameters<'static>>> {
        Rc::new(RefCell::new(DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: Blend::alpha_blending(),
            ..Default::default()
        }))
    }
}

impl Engine<'static> {
    pub fn init(self: &Rc<Self>, event_loop: EventLoop<()>) {
        self.scene.borrow().init();

        self.clone().run_event_loop(event_loop);
    }

    fn run_event_loop(self: Rc<Self>, event_loop: EventLoop<()>) {
        let mut old_frame_time = Instant::now();

        event_loop.run(move |event, _, control_flow| {
            let frame_time = Instant::now();
            let delta = frame_time.duration_since(old_frame_time);

            old_frame_time = frame_time;

            self.scene.borrow().update(&event, delta);

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
}
