use super::ColliderCallback;
use crate::{
    components::Transform,
    ecs::{self, AsAny, Component, Entity, Id, World},
};
use cgmath::{Vector2, Zero};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct ColliderRect {
    pub dims: Vector2<f32>,
    pub callback: Rc<RefCell<dyn ColliderCallback>>,
    pub active: bool,
}

impl ColliderRect {
    thread_local! {
        pub static ID: Id = ecs::id("collider_rect");
    }

    pub fn new<C>(dims: Vector2<f32>, callback: &Rc<RefCell<C>>, active: bool) -> Rc<RefCell<Self>>
    where
        C: ColliderCallback,
    {
        Rc::new(RefCell::new(Self {
            dims,
            callback: callback.clone(),
            active,
        }))
    }

    pub fn update(
        &mut self,
        world: &mut World,
        parent @ (id, _): &(Id, Rc<RefCell<Entity>>),
        transform: &Transform,
        components: &Vec<(
            (Id, Rc<RefCell<Entity>>),
            ((Id, Rc<RefCell<dyn AsAny>>), (Id, Rc<RefCell<dyn AsAny>>)),
        )>,
        delta: Duration,
    ) {
        if self.active {
            for (p @ (i, _), ((_, c), (_, t))) in components {
                if self.intersecting(transform, id, i, c.clone(), t.clone()) {
                    self.callback
                        .borrow_mut()
                        .callback(world, parent.clone(), p.clone(), delta);
                }
            }
        }
    }

    fn intersecting(
        &self,
        transform: &Transform,
        id: &Id,
        other_id: &Id,
        c: Rc<RefCell<dyn AsAny>>,
        t: Rc<RefCell<dyn AsAny>>,
    ) -> bool {
        if **id != **other_id {
            if let (Some(c), Some(t)) = (
                c.clone().borrow().as_any_ref().downcast_ref::<Self>(),
                t.clone().borrow().as_any_ref().downcast_ref::<Transform>(),
            ) {
                if c.active {
                    let (min, max) = self.dims_to_global(transform);
                    let points = c.dims_to_points(&t);

                    for p in points {
                        if p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y {
                            return true;
                        }
                    }
                }
            }
        }

        false
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
