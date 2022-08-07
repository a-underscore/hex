use crate::{AsAny, Id, World};

pub trait System: AsAny + 'static {
    fn id(&self) -> Id;

    fn on_init(&mut self, _world: &mut World) {}

    fn on_update(&mut self, _world: &mut World) {}
}
