use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder, NotCurrent,
    },
    Display, Surface,
};
use hecs::{ev::control::Control, Ev, SystemManager, World};

pub fn setup_display(
    wb: WindowBuilder,
    cb: ContextBuilder<'_, NotCurrent>,
) -> anyhow::Result<(EventLoop<()>, Display)> {
    let event_loop = EventLoop::new();
    let display = Display::new(wb, cb, &event_loop)?;

    Ok((event_loop, display))
}

pub fn init(
    event_loop: EventLoop<()>,
    mut world: World<'static>,
    mut system_manager: SystemManager<'static>,
) -> anyhow::Result<()> {
    fn update(
        mut control: Control,
        flow: &mut ControlFlow,
        world: &mut World<'static>,
        system_manager: &mut SystemManager<'static>,
    ) -> anyhow::Result<()> {
        system_manager.update(&mut Ev::Event(&mut control), world)?;

        if let Event::MainEventsCleared = &control.event {
            let mut target = world.display.draw();

            target.clear_color_and_depth(world.bg, 1.0);

            system_manager.update(&mut Ev::Draw((&mut control, &mut target)), world)?;

            target.finish()?;
        }

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = &control.event
        {
            *flow = ControlFlow::Exit;
        } else if let Some(control_flow) = control.flow {
            *flow = control_flow;
        } else {
            *flow = ControlFlow::Poll;
        }

        Ok(())
    }

    system_manager.init(&mut world)?;

    event_loop.run(move |event, _, control_flow| {
        if let Err(e) = update(
            Control::new(event),
            control_flow,
            &mut world,
            &mut system_manager,
        ) {
            eprintln!("{}", e);
        }
    });
}
