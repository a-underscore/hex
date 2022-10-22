use super::{AsAny, Component, Id, ToMut, ToRef, World};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub trait System: AsAny {
    fn update(
        &mut self,
        world: &Rc<RefCell<World>>,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()>;
}

impl ToRef for dyn System {
    fn to_ref<C>(&self) -> Option<&C>
    where
        C: Component + 'static,
    {
        self.as_any_ref().downcast_ref()
    }
}

impl ToMut for dyn System {
    fn to_mut<C>(&mut self) -> Option<&mut C>
    where
        C: Component + 'static,
    {
        self.as_any_mut().downcast_mut()
    }
}

pub type GenericSystem = (Id, Rc<RefCell<dyn System>>);
