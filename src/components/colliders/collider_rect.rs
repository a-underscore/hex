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
        parent: (Id, Rc<RefCell<Entity>>),
        transform: &Transform,
        components: &Vec<(
            (Id, Rc<RefCell<Entity>>),
            (Rc<RefCell<dyn Component>>, Rc<RefCell<dyn Component>>),
        )>,
    ) {
        let (parent_id, _) = parent.clone();

        for (p, (c, t)) in components {
            let (id, _) = p.clone();

            if *parent_id != *id {
                if let (Some(c), Some(t)) = (
                    c.borrow().as_any_ref().downcast_ref::<Self>(),
                    t.borrow_mut().as_any_mut().downcast_mut::<Transform>(),
                ) {
                    if self.intersecting(transform, c, t) {
                        self.callback
                            .borrow_mut()
                            .callback(parent.clone(), p.clone());
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
    ) -> bool {
        let (p1, p2) = {
            let transform = transform.get_transform();

            (
                (transform * self.dims.extend(1.0)).xy(),
                (transform * Vector3::zero()).xy(),
            )
        };

        for p in other.to_points() {
            let p = (other_transform.get_transform() * p.extend(1.0)).xy();

            if (p2.x - p1.x) * (p.y - p1.y) - (p.x - p1.x) * (p2.y - p1.y) != 0.0 {
                return true;
            }
        }

        return false;
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
