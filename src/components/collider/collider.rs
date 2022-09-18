use super::{Callback, Shape};
use crate::{
    components::Transform,
    ecs::{self, Component, Id},
    ecs::{AsAny, Entity, World},
};
use glium::glutin::event::Event;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
    time::Duration,
};

pub struct Collider {
    pub shape: Rc<RefCell<dyn Shape>>,
    pub callback: Rc<RefCell<dyn Callback>>,
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
        S: Shape + 'static,
        C: Callback + 'static,
    {
        Rc::new(RefCell::new(Self {
            shape: shape.clone(),
            callback: callback.clone(),
            active,
        }))
    }

    pub fn update(
        &mut self,
        parent @ (id, _): &(Id, Rc<RefCell<Entity>>),
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
            for (other_parent @ (other_id, _), ((_, other), (_, other_transform))) in components {
                if **id != **other_id {
                    if let (Some(mut other), Some(mut other_transform)) = (
                        RefMut::filter_map(other.try_borrow_mut()?, |c| {
                            c.as_any_mut().downcast_mut::<Collider>()
                        })
                        .ok(),
                        RefMut::filter_map(other_transform.try_borrow_mut()?, |t| {
                            t.as_any_mut().downcast_mut::<Transform>()
                        })
                        .ok(),
                    ) {
                        if {
                            let mut shape = self.shape.try_borrow_mut()?;

                            shape.intersecting(
                                parent,
                                transform,
                                &mut other,
                                other_parent,
                                &mut other_transform,
                                world,
                                delta,
                            )
                        } || other.shape.try_borrow_mut()?.intersecting(
                            other_parent,
                            &mut other_transform,
                            self,
                            parent,
                            transform,
                            world,
                            delta,
                        ) {
                            self.callback.try_borrow_mut()?.callback(
                                parent,
                                other_parent,
                                world,
                                event,
                                delta,
                            );
                        }
                    }
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
