use crate::ecs::World;
use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder, NotCurrent,
    },
    Display,
};
use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

pub fn setup_display(
    wb: WindowBuilder,
    cb: ContextBuilder<'_, NotCurrent>,
) -> anyhow::Result<(EventLoop<()>, Display)> {
    let event_loop = EventLoop::new();
    let display = Display::new(wb, cb, &event_loop)?;

    Ok((event_loop, display))
}

pub fn basic_display(name: &String, sample_count: u16) -> anyhow::Result<(EventLoop<()>, Display)> {
    let wb = WindowBuilder::new().with_title(name);
    let cb = ContextBuilder::new().with_multisampling(sample_count);

    setup_display(wb, cb)
}

pub fn init(world: Rc<RefCell<World>>, event_loop: EventLoop<()>) {
    fn update_world(
        world: Rc<RefCell<World>>,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()> {
        let mut world = world.try_borrow_mut()?;

        world.update_systems(&event, delta)
    }

    let mut old_frame_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let frame_time = Instant::now();
        let delta = frame_time.duration_since(old_frame_time);

        old_frame_time = frame_time;

        if let Err(e) = update_world(world.clone(), &event, delta) {
            println!("{:?}", e);
        }

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
