use crate::assets::Scene;
use glium::glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

pub fn init(scene: Rc<RefCell<Scene<'static>>>, event_loop: EventLoop<()>) -> anyhow::Result<()> {
    fn init_scene(scene: Rc<RefCell<Scene>>) -> anyhow::Result<()> {
        let world = scene.try_borrow()?.world.clone();
        let mut world = world.try_borrow_mut()?;

        world.init_systems()
    }

    fn update_scene(
        scene: Rc<RefCell<Scene>>,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()> {
        let world = scene.try_borrow()?.world.clone();
        let mut world = world.try_borrow_mut()?;

        world.update_systems(&event, delta)
    }

    init_scene(scene.clone())?;

    let mut old_frame_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let frame_time = Instant::now();
        let delta = frame_time.duration_since(old_frame_time);

        old_frame_time = frame_time;

        if let Err(e) = update_scene(scene.clone(), &event, delta) {
            println!("{}", e);
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
