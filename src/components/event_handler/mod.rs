mod callback;

pub use callback::Callback;

use crate::{ecs::Component, id};
use std::rc::Rc;

pub struct EventHandler {
    pub callback: Rc<dyn Callback>,
    pub active: bool,
}

impl EventHandler {
    pub fn new<F>(callback: Rc<F>, active: bool) -> Self
    where
        F: Callback + 'static,
    {
        Self { callback, active }
    }
}

impl Component for EventHandler {
    fn id() -> usize {
        id!()
    }
}
