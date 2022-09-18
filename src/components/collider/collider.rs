use super::{ColliderCallback, ColliderShape};
use crate::{
    components::Transform,
    ecs::{self, Component, Id},
    ecs::{AsAny, Entity, World},
};
use glium::glutin::event::Event;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct Collider {
    pub shape: Rc<RefCell<dyn ColliderShape>>,
    pub callback: Rc<RefCell<dyn ColliderCallback>>,
    pub active: bool,
}

impl Collider {
    thread_local! {
        static ID: Id = ecs::id("collider");
    }

    pub fn new<S, C>(
        shape: &Rc<RefCell<S>>,
        callback: &Rc<RefCell<C>>,
        active: bool,
    ) -> Rc<RefCell<Self>>
    where
        S: ColliderShape + 'static,
        C: ColliderCallback + 'static,
    {
        Rc::new(RefCell::new(Self {
            shape: shape.clone(),
            callback: callback.clone(),
            active,
        }))
    }

    pub fn update(
        &mut self,
        parent: &(Id, Rc<RefCell<Entity>>),
        transform: &mut Transform,
        components: &Vec<(
            (Id, Rc<RefCell<Entity>>),
            ((Id, Rc<RefCell<dyn AsAny>>), (Id, Rc<RefCell<dyn AsAny>>)),
        )>,
        world: &mut World,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()> {
        if self.active {
            for c @ (e, _) in components {
                if self
                    .shape
                    .try_borrow_mut()?
                    .intersecting(parent, transform, c, world, delta)
                {
                    self.callback
                        .try_borrow_mut()?
                        .callback(parent, e, world, event, delta);
                }
            }
        }

        Ok(())
    }
}

impl Component for Collider {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}
