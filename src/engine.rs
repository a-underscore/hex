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

pub fn setup_display(
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
    vsync: bool,
) -> anyhow::Result<(EventLoop<()>, Display)> {
    let wb = WindowBuilder::new().with_title(name);
    let cb = ContextBuilder::new()
        .with_multisampling(sample_count)
        .with_vsync(vsync);

    setup_display(wb, cb)
}

pub fn init(mut world: World<'static, 'static>, event_loop: EventLoop<()>) {
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
