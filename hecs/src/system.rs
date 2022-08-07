use crate::{AsAny, Id, World};

pub trait System: AsAny + 'static {
    fn id(&self) -> Id;

    fn on_init(&self, _world: &mut World) {}

    fn on_update(&self, _world: &mut World) {}
}
