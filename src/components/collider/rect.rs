use super::{Collider, Shape};
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
        &self,
        transform: &Transform,
        other: &Collider,
        other_transform: &Transform,
    ) -> anyhow::Result<Option<Vector2<f32>>> {
        if other.active {
            if let Some(points) = self.to_points().and_then(|p| {
                Some(
                    p.into_iter()
                        .map(|p| (transform.get_transform() * p.extend(1.0)).xy())
                        .collect::<Vec<_>>(),
                )
            }) {
                if let [a, b, c, _] = points.as_slice() {
                    if let Some(other_points) = other.shape.try_borrow_mut()?.to_points() {
                        for p in other_points {
                            let p = (other_transform.get_transform() * p.extend(1.0)).xy();
                            let ab = a - b;
                            let am = a - p;
                            let bc = b - c;
                            let bm = b - p;
                            let abam = ab.dot(am);
                            let abab = ab.dot(ab);
                            let bcbm = bc.dot(bm);
                            let bcbc = bc.dot(bc);

                            if 0.0 <= abam && abam <= abab && 0.0 <= bcbm && bcbm <= bcbc {
                                return Ok(Some(Vector2::new(abam / abab, bcbm / bcbc)));
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
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
    ) -> Option<Vector2<f32>> {
        self.try_intersecting(transform, other, other_transform)
            .unwrap_or(None)
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
