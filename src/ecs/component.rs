use super::AsAny;

pub trait Component {
    fn id() -> usize;
}

impl<'a, C> AsAny<'a> for C where C: Component + 'a {}
