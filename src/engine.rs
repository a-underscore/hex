use crate::ecs::{system_manager::Ev, world::World};
use cgmath::Vector4;
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
    mut world: World<'static, 'static>,
    event_loop: EventLoop<()>,
    display: Display,
    mut bg: Vector4<f32>,
) -> anyhow::Result<()> {
    fn update(
        world: &mut World,
        display: &Display,
        bg: &mut Vector4<f32>,
        event: &Event<()>,
        control_flow: &mut ControlFlow,
    ) -> anyhow::Result<()> {
        world.system_manager.update(
            display,
            &mut Ev::Event(event),
            &mut world.entity_manager,
            &mut world.component_manager,
        )?;

        if let Event::MainEventsCleared = event {
            let mut target = display.draw();

            target.clear_color_and_depth((*bg).into(), 1.0);

            world.system_manager.update(
                display,
                &mut Ev::Draw((event, &mut target, bg)),
                &mut world.entity_manager,
                &mut world.component_manager,
            )?;

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

    world.system_manager.init(
        &display,
        &mut world.entity_manager,
        &mut world.component_manager,
    )?;

    event_loop.run(move |event, _, control_flow| {
        update(&mut world, &display, &mut bg, &event, control_flow).unwrap();
    });
}
