use super::{Collider, Quantity, Shape};
use crate::{
    components::Transform,
    ecs::{Entity, Id, World},
};
use cgmath::{InnerSpace, Vector2, Zero};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct Rect {
    pub dims: Vector2<f32>,
}

impl Rect {
    pub fn new(dims: Vector2<f32>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { dims }))
    }

    fn try_intersecting(
        &mut self,
        transform: &mut Transform,
        other: &mut Collider,
        other_transform: &mut Transform,
    ) -> anyhow::Result<bool> {
        if other.active {
            match other
                .shape
                .try_borrow_mut()?
                .to_quantity()
                .to_global(other_transform)
            {
                (min, Quantity::Dims(max)) => {
                    let points = self.to_points();

                    for p in points {
                        let p = (transform.get_transform() * p.extend(1.0)).xy();

                        if p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y {
                            return Ok(true);
                        }
                    }
                }
                (pos, Quantity::Radius(radius)) => {
                    for p in self.to_points() {
                        if ((transform.get_transform() * p.extend(1.0)).xy() - pos).magnitude()
                            <= radius
                        {
                            return Ok(true);
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    pub fn to_points(&self) -> Vec<Vector2<f32>> {
        [
            self.dims,
            Vector2::new(0.0, self.dims.y),
            Vector2::zero(),
            Vector2::new(self.dims.x, 0.0),
        ]
        .to_vec()
    }
}

impl Shape for Rect {
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
        Quantity::Dims(self.dims)
    }
}
