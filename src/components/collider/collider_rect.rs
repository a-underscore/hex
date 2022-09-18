use super::{Collider, ColliderShape};
use crate::{
    components::Transform,
    ecs::{AsAny, Entity, Id, World},
};
use cgmath::{Vector2, Zero};
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
    time::Duration,
};

pub struct ColliderRect {
    pub dims: Vector2<f32>,
}

impl ColliderRect {
    pub fn new(dims: Vector2<f32>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { dims }))
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

    fn try_intersecting(
        &self,
        transform: &mut Transform,
        id: &Id,
        other_id: &Id,
        (c, t): &(Rc<RefCell<dyn AsAny>>, Rc<RefCell<dyn AsAny>>),
    ) -> anyhow::Result<bool> {
        if **id != **other_id {
            if let (Some(c), Some(mut t)) = (
                Ref::filter_map(c.try_borrow()?, |c| {
                    c.as_any_ref().downcast_ref::<Collider>()
                })
                .ok(),
                RefMut::filter_map(t.try_borrow_mut()?, |t| {
                    t.as_any_mut().downcast_mut::<Transform>()
                })
                .ok(),
            ) {
                if c.active {
                    if let Some(points) = c.shape.try_borrow_mut()?.to_points(&mut t) {
                        let (min, max) = self.to_global(&transform);

                        for p in points {
                            if p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }

        Ok(false)
    }
}

impl ColliderShape for ColliderRect {
    fn intersecting(
        &mut self,
        (id, _): &(Id, Rc<RefCell<Entity>>),
        transform: &mut Transform,
        ((other_id, _), ((_, other_collider), (_, other_transform))): &(
            (Id, Rc<RefCell<Entity>>),
            ((Id, Rc<RefCell<dyn AsAny>>), (Id, Rc<RefCell<dyn AsAny>>)),
        ),
        _: &mut World,
        _: Duration,
    ) -> bool {
        match self.try_intersecting(
            transform,
            id,
            &other_id,
            &(other_collider.clone(), other_transform.clone()),
        ) {
            Ok(i) => i,
            Err(_) => false,
        }
    }

    fn to_points(&mut self, transform: &mut Transform) -> Option<Vec<Vector2<f32>>> {
        let transform = transform.get_transform();

        Some(
            [
                self.dims,
                Vector2::new(0.0, self.dims.y),
                Vector2::zero(),
                Vector2::new(self.dims.x, 0.0),
            ]
            .into_iter()
            .map(|p| (transform * p.extend(1.0)).xy())
            .collect(),
        )
    }
}
