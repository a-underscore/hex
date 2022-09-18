use super::{Collider, Quantity, Shape};
use crate::{
    components::Transform,
    ecs::{Entity, Id, World},
};
use cgmath::{InnerSpace, Vector2};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { radius }))
    }

    fn try_intersecting(
        &mut self,
        transform: &mut Transform,
        other: &mut Collider,
        other_transform: &mut Transform,
    ) -> anyhow::Result<bool> {
        if other.active {
            if let ((pos, Quantity::Radius(radius)), (other_pos, Quantity::Radius(other_radius))) = (
                self.to_quantity().to_global(transform),
                other
                    .shape
                    .try_borrow_mut()?
                    .to_quantity()
                    .to_global(other_transform),
            ) {
                if (pos + Vector2::new(radius, 0.0) - other_pos).magnitude() <= other_radius {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

impl Shape for Circle {
    fn intersecting(
        &mut self,
        _: &(Id, Rc<RefCell<Entity>>),
        transform: &mut Transform,
        other: &mut Collider,
        _: &(Id, Rc<RefCell<Entity>>),
        other_transform: &mut Transform,
        _: &mut World,
        _: Duration,
    ) -> bool {
        self.try_intersecting(transform, other, other_transform)
            .unwrap_or(false)
    }

    fn to_quantity(&self) -> Quantity {
        Quantity::Radius(self.radius)
    }
}
