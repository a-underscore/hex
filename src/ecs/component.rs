use super::AsAny;

pub trait Component: AsAny {
    fn id() -> usize;
}

impl<C> AsAny for C where C: Component {}
