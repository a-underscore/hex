use super::CollisionCallback;
use crate::{
    components::Transform,
    ecs::{self, Component, Id},
};
use std::{cell::RefCell, rc::Rc};

pub struct ColliderQuad {
    pub callback: Rc<RefCell<dyn CollisionCallback>>,
}

impl ColliderQuad {
    thread_local! {
        pub static ID: Id = ecs::id("collider_rect");
    }

    pub fn new<C>(callback: Rc<RefCell<C>>) -> Rc<RefCell<Self>>
    where
        C: CollisionCallback,
    {
        Rc::new(RefCell::new(Self { callback }))
    }

    pub fn update(&mut self, _transform: &Transform) {}
}

impl Component for ColliderQuad {
    fn get_id(&self) -> Id {
        ecs::tid(&Self::ID)
    }
}
