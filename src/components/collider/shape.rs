use super::Collider;
use crate::{
    components::Transform,
    ecs::{AsAny, Entity, Id, World},
};
use cgmath::Vector2;
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
    ) -> Option<Vector2<f32>>;

    fn to_points(&self) -> Option<Vec<Vector2<f32>>> {
        None
    }
}
