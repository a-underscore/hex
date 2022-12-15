mod callback;

pub use callback::Callback;

use crate::{ecs::Component, id};

pub struct EventHandler<'a> {
    pub callback: Box<dyn Callback<'a>>,
    pub active: bool,
}

impl<'a> EventHandler<'a> {
    pub fn new<C>(callback: C, active: bool) -> Self
    where
        C: Callback<'a>,
    {
        Self {
            callback: Box::new(callback),
            active,
        }
    }
}

impl<'a> Component for EventHandler<'a> {
    fn id() -> usize {
        id!()
    }
}
