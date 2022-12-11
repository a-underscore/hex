use crate::ecs::{self, Id, Type, World};
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
) -> anyhow::Result<(EventLoop<()>, Type<Display>)> {
    let event_loop = EventLoop::new();
    let display = ecs::new(Display::new(wb, cb, &event_loop)?);

    Ok((event_loop, display))
}

pub fn basic_display(
    name: &Id,
    sample_count: u16,
) -> anyhow::Result<(EventLoop<()>, Type<Display>)> {
    let wb = WindowBuilder::new().with_title(name);
    let cb = ContextBuilder::new().with_multisampling(sample_count);

    setup_display(wb, cb)
}

pub fn init(world: Type<World>, event_loop: EventLoop<()>) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Err(e) = ecs::update(&world, &event) {
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
