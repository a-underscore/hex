use super::{Collider, Shape};
use crate::{
    components::Transform,
    ecs::{Entity, Id, World},
};
use cgmath::{Vector2, Zero};
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
            if let Some(points) = other.shape.try_borrow_mut()?.to_points() {
                let (min, max) = self.to_global(transform);

                for p in points {
                    let p = (other_transform.get_transform() * p.extend(1.0)).xy();

                    if p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    fn to_global(&self, transform: &Transform) -> (Vector2<f32>, Vector2<f32>) {
        let (p1, p2) = {
            let transform = transform.get_transform();
            (
                (transform * Vector2::zero().extend(1.0)).xy(),
                (transform * self.dims.extend(1.0)).xy(),
            )
        };
        let (min_x, max_x) = if p1.x < p2.x {
            (p1.x, p2.x)
        } else {
            (p2.x, p1.x)
        };
        let (min_y, max_y) = if p1.y < p2.y {
            (p1.y, p2.y)
        } else {
            (p2.y, p1.y)
        };

        (Vector2::new(min_x, min_y), Vector2::new(max_x, max_y))
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

    fn to_points(&self) -> Option<Vec<Vector2<f32>>> {
        Some(
            [
                self.dims,
                Vector2::new(0.0, self.dims.y),
                Vector2::zero(),
                Vector2::new(self.dims.x, 0.0),
            ]
            .to_vec(),
        )
    }
}
