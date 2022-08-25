use super::{ColliderCallback, ColliderShape};
use crate::{
    components::Transform,
    ecs::{self, Component, Id},
    ecs::{AsAny, Entity, World},
};
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};

pub struct Collider {
    pub shape: Rc<RefCell<dyn ColliderShape>>,
    pub callback: Rc<RefCell<dyn ColliderCallback>>,
    pub active: bool,
}

impl Collider {
    thread_local! {
        static ID: Id = ecs::id("collider_rect");
    }

    pub fn new<S, C>(
        shape: &Rc<RefCell<S>>,
        callback: &Rc<RefCell<C>>,
        active: bool,
    ) -> Rc<RefCell<Self>>
    where
        S: ColliderShape,
        C: ColliderCallback,
    {
        Rc::new(RefCell::new(Self {
            shape: shape.clone(),
            callback: callback.clone(),
            active,
        }))
    }

    pub fn update(
        &mut self,
        world: &mut World,
        parent: &(Id, Rc<RefCell<Entity>>),
        transform: &Transform,
        components: &Vec<(
            (Id, Rc<RefCell<Entity>>),
            ((Id, Rc<RefCell<dyn AsAny>>), (Id, Rc<RefCell<dyn AsAny>>)),
        )>,
        delta: Duration,
    ) {
        if self.active {
            for i in self
                .shape
                .borrow_mut()
                .get_intersecting(world, parent, transform, components, delta)
            {
                self.callback
                    .borrow_mut()
                    .callback(world, parent.clone(), i, delta);
            }
        }
    }
}

impl Component for Collider {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}
