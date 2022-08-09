use crate::{AsAny, Id};

pub trait Component: 'static + AsAny {
    fn get_id(&self) -> Id;
}
