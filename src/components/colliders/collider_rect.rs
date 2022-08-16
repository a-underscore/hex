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
        parent_id: Id,
        parent: &mut Entity,
        transform: &Transform,
        components: &Vec<(
            (Id, Rc<RefCell<Entity>>),
            (Rc<RefCell<dyn Component>>, Rc<RefCell<dyn Component>>),
        )>,
    ) {
        for ((id, e), (c, t)) in components {
            if *parent_id != **id {
                if let (Some(c), Some(t)) = (
                    c.borrow().as_any_ref().downcast_ref::<Self>(),
                    t.borrow_mut().as_any_mut().downcast_mut::<Transform>(),
                ) {
                    if let Some(intersecting) = self.find_intersecting(transform, c, t) {
                        self.callback.borrow_mut().callback(
                            parent,
                            &mut e.borrow_mut(),
                            &intersecting,
                        );
                    }
                }
            }
        }
    }

    fn find_intersecting(
        &self,
        transform: &Transform,
        other: &Self,
        other_transform: &Transform,
    ) -> Option<Vec<Vector2<f32>>> {
        let (min, max) = {
            let transform = transform.get_transform();

            (
                (transform * Vector3::zero()).xy(),
                (transform * self.dims.extend(0.0)).xy(),
            )
        };

        let vec = other
            .to_points()
            .into_iter()
            .filter(|p| {
                let p = (other_transform.get_transform() * p.extend(0.0)).xy();

                p.x >= min.x && p.y >= min.y && p.x <= max.x && p.y <= max.y
            })
            .collect::<Vec<_>>();

        (!vec.is_empty()).then(|| vec)
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
