use crate::{
    assets::Scene,
    components::{Sprite, SPRITE_ID},
    ecs::{self, Component, Entity, ENTITY_ID},
};
use glium::{
    draw_parameters::{Blend, DepthTest},
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder, NotCurrent,
    },
    Depth, Display, DrawParameters, Frame, Surface,
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

    fn draw_sprites(&self, entity: &Entity, target: &mut Frame) -> anyhow::Result<()> {
        for sprite in entity.get_all::<Sprite>(&ecs::tid(&SPRITE_ID)) {
            sprite.draw(entity.clone(), self.clone(), target)?;
        }

        for entity in entity.get_all::<Entity>(&ecs::tid(&ENTITY_ID)) {
            self.clone().draw_sprites(entity.as_ref(), target)?;
        }

        Ok(())
    }
}

impl Engine<'static> {
    pub fn init(self: &Rc<Self>, event_loop: EventLoop<()>) {
        self.scene.borrow().root.clone().on_init(None);

        self.clone().run_event_loop(event_loop);
    }

    fn run_event_loop(self: Rc<Self>, event_loop: EventLoop<()>) {
        let mut last_frame_time = Instant::now();

        event_loop.run(move |event, _, control_flow| {
            let current_frame_time = Instant::now();
            let delta = current_frame_time.duration_since(last_frame_time);

            last_frame_time = current_frame_time;

            self.scene
                .borrow()
                .root
                .clone()
                .on_update(None, &event, delta);

            let mut target = self.display.draw();

            target.clear_color_and_depth(self.scene.borrow().bg.into(), 1.0);

            self.draw_sprites(self.scene.borrow().root.as_ref(), &mut target)
                .unwrap();

            target.finish().unwrap();

            *control_flow = ControlFlow::Wait;

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
