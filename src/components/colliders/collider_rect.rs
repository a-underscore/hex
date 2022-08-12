use super::CollisionCallback;
use crate::{
    components::Transform,
    ecs::{self, Component, Entity, Id},
};
use cgmath::{Vector2, Vector3, Zero};
use std::{cell::RefCell, rc::Rc};

pub struct ColliderRect {
    pub dims: Vector2<f32>,
    pub callback: Rc<RefCell<dyn CollisionCallback>>,
}

impl ColliderRect {
    thread_local! {
        pub static ID: Id = ecs::id("collider_rect");
    }

    pub fn new<C>(dims: Vector2<f32>, callback: Rc<RefCell<C>>) -> Rc<RefCell<Self>>
    where
        C: CollisionCallback,
    {
        Rc::new(RefCell::new(Self { dims, callback }))
    }

    pub fn update(
        &mut self,
        parent: (Id, &mut Entity),
        transform: &Transform,
        components: &Vec<(
            (Id, Rc<RefCell<Entity>>),
            (Rc<RefCell<dyn Component>>, Rc<RefCell<dyn Component>>),
        )>,
    ) {
        let (_, entity) = parent;

        for ((_, e), (c, t)) in components {
            if *self.get_id() != *c.borrow().get_id() {
                if let (Some(c), Some(t)) = (
                    c.borrow().as_any_ref().downcast_ref::<Self>(),
                    t.borrow_mut().as_any_mut().downcast_mut::<Transform>(),
                ) {
                    if self.detect(transform, c, t) {
                        self.callback
                            .borrow_mut()
                            .callback(entity, &mut e.borrow_mut());
                    }
                }
            }
        }
    }

    fn detect(&self, transform: &Transform, other: &Self, other_transform: &Transform) -> bool {
        let min = (transform.get_transform() * Vector3::zero()).xy();
        let max = (transform.get_transform() * self.dims.extend(0.0)).xy();
        let other_points = other
            .to_points()
            .iter()
            .map(|p| (other_transform.get_transform() * p.extend(0.0)).xy())
            .collect::<Vec<_>>();

        for p in other_points {
            if p.x >= min.x && p.y >= min.y && p.x <= max.x && p.y <= max.y {
                return true;
            }
        }

        false
    }

    fn to_points(&self) -> [Vector2<f32>; 4] {
        [
            self.dims,
            Vector2::new(0.0, self.dims.y),
            Vector2::zero(),
            Vector2::new(self.dims.x, 0.0),
        ]
    }
}

impl Component for ColliderRect {
    fn get_id(&self) -> Id {
        ecs::tid(&Self::ID)
    }
}
