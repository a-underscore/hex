use super::AsAny;
use std::{any::Any, cell::RefCell, rc::Rc};

pub trait Component: 'static {}

impl<C> AsAny for Rc<RefCell<C>>
where
    C: Component,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
