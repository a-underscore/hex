use crate::ecs::{Component, Entity};

pub trait CollisionCallback: 'static + Component {
    fn callback(&mut self, parent: &mut Entity, other: &mut Entity);
}
