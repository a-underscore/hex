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
use std::rc::Rc;

pub fn setup_display<'a>(
    wb: WindowBuilder,
    cb: ContextBuilder<'a, NotCurrent>,
) -> anyhow::Result<(EventLoop<()>, Rc<Display>)> {
    let event_loop = EventLoop::new();
    let display = Rc::new(Display::new(wb, cb, &event_loop)?);

    Ok((event_loop, display))
}

pub fn basic_display<S>(
    name: S,
    sample_count: u16,
    vsync: bool,
) -> anyhow::Result<(EventLoop<()>, Rc<Display>)>
where
    S: Into<String>,
{
    let wb = WindowBuilder::new().with_title(name);
    let cb = ContextBuilder::new()
        .with_multisampling(sample_count)
        .with_vsync(vsync);

    setup_display(wb, cb)
}

pub fn init(mut world: World<'static, 'static>, event_loop: EventLoop<()>) -> anyhow::Result<()> {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Err(e) = world.update(&event) {
            eprintln!("{:?}", e);
        }

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
