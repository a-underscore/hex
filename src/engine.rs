use crate::ecs::{
    system_manager::{Ev, SystemManager},
    world::World,
};
use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder, NotCurrent,
    },
    Display, Surface,
};

pub fn setup_display(
    wb: WindowBuilder,
    cb: ContextBuilder<'_, NotCurrent>,
) -> anyhow::Result<(EventLoop<()>, Display)> {
    let event_loop = EventLoop::new();
    let display = Display::new(wb, cb, &event_loop)?;

    Ok((event_loop, display))
}

pub fn init(
    mut world: World<'static>,
    mut system_manager: SystemManager<'static>,
    event_loop: EventLoop<()>,
) -> anyhow::Result<()> {
    fn update(
        event: &Event<()>,
        control_flow: &mut ControlFlow,
        world: &mut World,
        system_manager: &mut SystemManager<'static>,
    ) -> anyhow::Result<()> {
        system_manager.update(&mut Ev::Event(event), world)?;

        if let Event::MainEventsCleared = event {
            let mut target = world.display.draw();

            target.clear_color_and_depth(world.bg.into(), 1.0);

            system_manager.update(&mut Ev::Draw((event, &mut target)), world)?;

            target.finish()?;
        } else if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        } else {
            *control_flow = ControlFlow::Poll;
        }

        Ok(())
    }

    system_manager.init(&mut world)?;

    event_loop.run(move |event, _, control_flow| {
        update(&event, control_flow, &mut world, &mut system_manager).unwrap();
    });
}
