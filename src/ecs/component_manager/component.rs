use super::Generic;

pub trait Component {
    fn id() -> usize;
}

impl<'a, C> Generic<'a> for C where C: Component + 'a {}
