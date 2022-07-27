use crate::{
    assets::Scene,
    components::{event_handler::EVENT_HANDLER_ID, sprite::SPRITE_ID, EventHandler, Sprite},
    ecs::{self, entity::ENTITY_ID, Component, Entity},
};
use glium::{
    draw_parameters::Blend,
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder, NotCurrent,
    },
    Display, DrawParameters, Frame, Surface,
};
use std::rc::Rc;

pub struct Engine {
    pub display: Display,
    pub scene: Rc<Scene>,
}

impl Engine {
    pub fn display(
        wb: WindowBuilder,
        cb: ContextBuilder<'_, NotCurrent>,
    ) -> anyhow::Result<(EventLoop<()>, Display)> {
        let event_loop = EventLoop::new();
        let display = Display::new(wb, cb, &event_loop)?;

        Ok((event_loop, display))
    }

    pub fn new(display: Display, scene: Rc<Scene>) -> anyhow::Result<Rc<Self>> {
        Ok(Rc::new(Self { display, scene }))
    }

    pub fn init(self: &Rc<Self>, event_loop: EventLoop<()>) {
        self.scene.root.on_init(None);

        self.clone().run_event_loop(event_loop);
    }

    fn run_event_loop(self: Rc<Engine>, event_loop: EventLoop<()>) {
        event_loop.run(move |ev, _, control_flow| {
            self.scene.root.on_update(None);

            let mut target = self.display.draw();

            target.clear_color_and_depth(self.scene.bg.into(), 1.0);

            let draw_params = DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },

                blend: Blend::alpha_blending(),
                ..Default::default()
            };

            Self::handle_events(self.scene.root.as_ref(), &ev);
            Self::draw_sprites(self.scene.root.as_ref(), &mut target, &draw_params).unwrap();

            target.finish().unwrap();

            *control_flow = ControlFlow::Wait;

            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;

                        return;
                    }
                    _ => return,
                },
                _ => (),
            }
        });
    }

    fn draw_sprites(
        entity: &Entity,
        target: &mut Frame,
        draw_params: &DrawParameters,
    ) -> anyhow::Result<()> {
        for sprite in entity.get_all::<Sprite>(ecs::id(SPRITE_ID)) {
            sprite.draw(target, draw_params)?;
        }

        for entity in entity.get_all::<Entity>(ecs::id(ENTITY_ID)) {
            Self::draw_sprites(entity.as_ref(), target, draw_params)?;
        }

        Ok(())
    }

    fn handle_events(entity: &Entity, event: &Event<()>) {
        for handler in entity.get_all::<EventHandler>(ecs::id(EVENT_HANDLER_ID)) {
            handler.handle(Some(entity), event);
        }

        for entity in entity.get_all::<Entity>(ecs::id(ENTITY_ID)) {
            Self::handle_events(entity.as_ref(), event);
        }
    }
}
