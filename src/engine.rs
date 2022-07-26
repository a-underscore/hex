use glium::{glutin, Display, Surface};
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, NotCurrent,
};
use std::sync::Arc;

pub struct Engine {
    pub display: Display,
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

    pub fn new(display: Display) -> anyhow::Result<Arc<Self>> {
        Ok(Arc::new(Self { display }))
    }

    pub fn init(self: Arc<Self>, event_loop: EventLoop<()>) {
        event_loop.run(move |ev, _, control_flow| {
            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);
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
