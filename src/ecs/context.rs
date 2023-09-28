use super::{ev::Control, ComponentManager, EntityManager, Ev, SystemManager};
use glium::{
    glutin::{
        event::Event,
        event_loop::{ControlFlow, EventLoop},
    },
    Display, Surface,
};

#[derive(Clone)]
pub struct Context {
    pub display: Display,
    pub bg: [f32; 4],
}

impl Context {
    pub fn new(display: Display, bg: [f32; 4]) -> Self {
        Self { display, bg }
    }

    pub fn init(
        mut self,
        event_loop: EventLoop<()>,
        (mut em, mut cm): (EntityManager, ComponentManager<'static>),
        mut sm: SystemManager<'static>,
    ) -> anyhow::Result<()> {
        sm.init(&mut self, (&mut em, &mut cm))?;

        event_loop.run(move |event, _, cf| {
            if let Err(e) = self.update(Control::new(event), cf, (&mut em, &mut cm), &mut sm) {
                eprintln!("{}", e);
            }
        })
    }

    pub fn update(
        &mut self,
        mut control: Control,
        cf: &mut ControlFlow,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
        sm: &mut SystemManager,
    ) -> anyhow::Result<()> {
        match control.event {
            Event::RedrawRequested(window_id)
                if window_id == self.display.gl_window().window().id() =>
            {
                let mut target = self.display.draw();

                target.clear_color_and_depth(
                    {
                        let [r, g, b, a] = self.bg;

                        (r, g, b, a)
                    },
                    1.0,
                );

                sm.update(&mut Ev::Draw((&mut control, &mut target)), self, (em, cm))?;

                target.finish()?;
            }
            _ => {
                sm.update(&mut Ev::Event(&mut control), self, (em, cm))?;

                if let Event::MainEventsCleared = control.event {
                    self.display.gl_window().window().request_redraw();
                }
            }
        }

        *cf = match control {
            Control {
                flow: Some(flow),
                event: _,
            } => flow,
            _ => ControlFlow::Poll,
        };

        Ok(())
    }
}
