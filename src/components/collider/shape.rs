use super::{Collider, Quantity};
use crate::{
    components::Transform,
    ecs::{AsAny, Entity, Id, World},
};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub trait Shape: AsAny {
    fn intersecting(
        &mut self,
        parent: &(Id, Rc<RefCell<Entity>>),
        transform: &mut Transform,
        other: &mut Collider,
        other_parent: &(Id, Rc<RefCell<Entity>>),
        other_transform: &mut Transform,
        world: &mut World,
        delta: Duration,
    ) -> bool;

    fn to_quantity(&self) -> Quantity;
}
