use std::sync::{Arc, RwLock};
use winit::event::Event;

pub struct Control {
    pub event: Event<()>,
    pub exit: bool,
}

impl Control {
    pub fn new(event: Event<()>) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self { event, exit: false }))
    }
}
