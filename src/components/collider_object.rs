use crate::{
    components::Transform,
    ecs::{self, Component, Id},
    systems::PhysicsSystem,
};
use rapier2d::prelude::*;
use std::{cell::RefCell, rc::Rc};

thread_local! {
    pub static COLLIDER_OBJECT_ID: Id = ecs::id("collider");
}

pub struct ColliderObject {
    collider: Collider,
    _instance: Option<(Rc<RefCell<PhysicsSystem>>, ColliderHandle)>,
}

impl ColliderObject {
    pub fn new(collider: Collider) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            collider,
            _instance: None,
        }))
    }

    pub fn update(&self, _transform: &Transform) {}

    pub fn get_collider<'a>(&'a self) -> &'a Collider {
        &self.collider
    }
}

impl Component for ColliderObject {
    fn get_id(&self) -> Id {
        ecs::tid(&COLLIDER_OBJECT_ID)
    }
}
