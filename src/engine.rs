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

pub fn init(scene: Rc<RefCell<Scene<'static>>>, event_loop: EventLoop<()>) {
    fn init_scene(scene: Rc<RefCell<Scene>>) {
        let world = scene.borrow().world.clone();

        world.borrow_mut().init_systems();
    }

    fn update_scene(scene: Rc<RefCell<Scene>>, event: &Event<()>, delta: Duration) {
        let world = scene.borrow().world.clone();

        world.borrow_mut().update_systems(&event, delta);
    }

    init_scene(scene.clone());

    let mut old_frame_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let frame_time = Instant::now();
        let delta = frame_time.duration_since(old_frame_time);

        old_frame_time = frame_time;

        update_scene(scene.clone(), &event, delta);

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
