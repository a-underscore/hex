use crate::{
    assets::Scene,
    components::{event_handler::EVENT_HANDLER_ID, EventHandler, Sprite, SPRITE_ID},
    ecs::{self, Entity, ENTITY_ID},
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
use std::{cell::RefCell, rc::Rc};

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

    fn handle_events(entity: Rc<Entity>, event: &Event<()>) {
        for handler in entity.get_all::<EventHandler>(ecs::tid(&EVENT_HANDLER_ID)) {
            handler.handle(Some(entity.clone()), event);
        }

        for entity in entity.get_all::<Entity>(ecs::tid(&ENTITY_ID)) {
            Self::handle_events(entity.clone(), event);
        }
    }
    fn draw_sprites(self: Rc<Self>, entity: Rc<Entity>, target: &mut Frame) -> anyhow::Result<()> {
        for sprite in entity.get_all::<Sprite>(ecs::tid(&SPRITE_ID)) {
            sprite.draw(entity.clone(), self.clone(), target)?;
        }

        for entity in entity.get_all::<Entity>(ecs::tid(&ENTITY_ID)) {
            self.clone().draw_sprites(entity.clone(), target)?;
        }

        Ok(())
    }
}

impl Engine<'static> {
    pub fn init(self: &Rc<Self>, event_loop: EventLoop<()>) {
        self.scene.borrow().init();

        self.clone().run_event_loop(event_loop);
    }

    fn run_event_loop(self: Rc<Self>, event_loop: EventLoop<()>) {
        event_loop.run(move |ev, _, control_flow| {
            Self::handle_events(self.scene.borrow().root.clone(), &ev);

            let mut target = self.display.draw();

            target.clear_color_and_depth(self.scene.borrow().bg.into(), 1.0);

            self.clone()
                .draw_sprites(self.scene.borrow().root.clone(), &mut target)
                .unwrap();

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
}
