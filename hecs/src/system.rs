use crate::{Id, World};

pub trait System: 'static {
    fn id(&self) -> Id;

    fn on_init(&mut self, _world: &mut World) {}

    fn on_update(&mut self, _world: &mut World) {}
}
