use crate::{AsAny, Id};

pub trait Component: AsAny {
    fn get_id() -> Id;
}
