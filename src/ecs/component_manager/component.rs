use super::Generic;
use crate::ecs::Id;

pub trait Component {
    fn id() -> Id;
}

impl<'a, C> Generic<'a> for C where C: Component + 'a {}
