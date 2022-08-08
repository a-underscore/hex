use crate::{AsAny, Id};

pub trait Component: 'static + AsAny {
    fn id(&self) -> Id;
}
