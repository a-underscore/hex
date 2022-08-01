use crate::{
    assets::Scene,
    components::{event_handler::EVENT_HANDLER_ID, EventHandler, Sprite, SPRITE_ID},
    ecs::{Entity, ENTITY_ID},
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

    fn draw_sprites(&self, entity: &Entity, target: &mut Frame) -> anyhow::Result<()> {
        for sprite in entity.get_all::<Sprite>(SPRITE_ID.with(|id| id.clone())) {
            sprite.draw(entity, self, target)?;
        }

        for entity in entity.get_all::<Entity>(ENTITY_ID.with(|id| id.clone())) {
            self.draw_sprites(entity.as_ref(), target)?;
        }

        Ok(())
    }

    fn handle_events(entity: &Entity, event: &Event<()>) {
        for handler in entity.get_all::<EventHandler>(EVENT_HANDLER_ID.with(|id| id.clone())) {
            handler.handle(Some(entity), event);
        }

        for entity in entity.get_all::<Entity>(ENTITY_ID.with(|id| id.clone())) {
            Self::handle_events(entity.as_ref(), event);
        }
    }
}

impl Engine<'static> {
    pub fn init(self: &Rc<Self>, event_loop: EventLoop<()>) {
        self.scene.borrow().init();

        self.clone().run_event_loop(event_loop);
    }

    fn run_event_loop(self: Rc<Self>, event_loop: EventLoop<()>) {
        event_loop.run(move |ev, _, control_flow| {
            self.scene.borrow().update();

            let mut target = self.display.draw();

            target.clear_color_and_depth(self.scene.borrow().bg.into(), 1.0);

            Self::handle_events(self.scene.borrow().root.as_ref(), &ev);

            self.draw_sprites(self.scene.borrow().root.as_ref(), &mut target)
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
