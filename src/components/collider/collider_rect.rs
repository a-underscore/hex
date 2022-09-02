use super::{Collider, ColliderShape};
use crate::{
    components::Transform,
    ecs::{AsAny, Entity, Id, World},
};
use cgmath::{Vector2, Zero};
use std::{
    cell::{Ref, RefCell},
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

    fn intersecting(
        &self,
        transform: &Transform,
        id: &Id,
        other_id: &Id,
        c: Rc<RefCell<dyn AsAny>>,
        t: Rc<RefCell<dyn AsAny>>,
    ) -> anyhow::Result<bool> {
        if **id != **other_id {
            if let (Some(c), Some(t)) = (
                Ref::filter_map(c.try_borrow()?, |c| {
                    c.as_any_ref().downcast_ref::<Collider>()
                })
                .ok(),
                Ref::filter_map(t.try_borrow()?, |t| {
                    t.as_any_ref().downcast_ref::<Transform>()
                })
                .ok(),
            ) {
                if c.active {
                    if let Some(c) = Ref::filter_map(c.shape.try_borrow()?, |s| {
                        s.as_any_ref().downcast_ref::<Self>()
                    })
                    .ok()
                    {
                        let (min, max) = self.dims_to_global(transform);
                        let points = c.dims_to_points(&t);

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

    fn dims_to_global(&self, transform: &Transform) -> (Vector2<f32>, Vector2<f32>) {
        let transform = transform.get_transform();
        let p1 = (transform * Vector2::zero().extend(1.0)).xy();
        let p2 = (transform * self.dims.extend(1.0)).xy();
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

    fn dims_to_points(&self, transform: &Transform) -> Vec<Vector2<f32>> {
        let transform = transform.get_transform();

        [
            self.dims,
            Vector2::new(0.0, self.dims.y),
            Vector2::zero(),
            Vector2::new(self.dims.x, 0.0),
        ]
        .into_iter()
        .map(|p| (transform * p.extend(1.0)).xy())
        .collect()
    }
}

impl ColliderShape for ColliderRect {
    fn get_intersecting(
        &mut self,
        _: &mut World,
        (id, _): &(Id, Rc<RefCell<Entity>>),
        transform: &Transform,
        components: &Vec<(
            (Id, Rc<RefCell<Entity>>),
            ((Id, Rc<RefCell<dyn AsAny>>), (Id, Rc<RefCell<dyn AsAny>>)),
        )>,
        _: Duration,
    ) -> Vec<(Id, Rc<RefCell<Entity>>)> {
        let mut intersecting = Vec::new();

        for (p @ (i, _), ((_, c), (_, t))) in components {
            if let Ok(true) = self.intersecting(transform, id, i, c.clone(), t.clone()) {
                intersecting.push(p.clone());
            }
        }

        intersecting
    }
}
