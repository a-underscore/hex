use super::{AsAny, Id};

pub trait Component {
    fn id() -> Id;
}

impl<C> AsAny for C where C: Component {}
