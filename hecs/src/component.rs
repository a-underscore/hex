use crate::{AsAny, Id};

pub trait Component: AsAny + 'static {
    fn get_id(&self) -> Id;
}
