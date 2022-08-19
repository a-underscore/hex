use super::ColliderCallback;
use crate::{
    components::Transform,
    ecs::{self, AsAny, Component, Entity, Id},
};
use cgmath::{Vector2, Zero};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct ColliderRect {
    pub dims: Vector2<f32>,
    pub callback: Rc<RefCell<dyn ColliderCallback>>,
}

impl ColliderRect {
    thread_local! {
        pub static ID: Id = ecs::id("collider_rect");
    }

    pub fn new<C>(dims: Vector2<f32>, callback: &Rc<RefCell<C>>) -> Rc<RefCell<Self>>
    where
        C: ColliderCallback,
    {
        Rc::new(RefCell::new(Self {
            dims,
            callback: callback.clone(),
        }))
    }

    pub fn update(
        &mut self,
        parent @ (id, _): &(Id, Rc<RefCell<Entity>>),
        transform: &Transform,
        components: &Vec<(
            (Id, Rc<RefCell<Entity>>),
            ((Id, Rc<RefCell<dyn AsAny>>), (Id, Rc<RefCell<dyn AsAny>>)),
        )>,
        delta: Duration,
    ) {
        for (p @ (i, _), ((_, c), (_, t))) in components {
            if **id != **i {
                if let (Some(c), Some(t)) = (
                    c.borrow().as_any_ref().downcast_ref::<Self>(),
                    t.borrow_mut().as_any_mut().downcast_mut::<Transform>(),
                ) {
                    if let Some(i) = self.intersecting(transform, c, t) {
                        self.callback
                            .borrow_mut()
                            .callback(parent.clone(), p.clone(), &i, delta);
                    }
                }
            }
        }
    }

    fn intersecting(
        &self,
        transform: &Transform,
        other: &Self,
        other_transform: &Transform,
    ) -> Option<Vec<Vector2<f32>>> {
        let (min, max) = self.dims_to_global(transform);
        let points = other.dims_to_points(&other_transform);
        let mut intersecting = Vec::new();

        for p in points {
            if p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y {
                intersecting.push(p);
            }
        }

        (!intersecting.is_empty()).then(|| intersecting)
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

impl Component for ColliderRect {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}
