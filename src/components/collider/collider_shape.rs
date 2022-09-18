use crate::{
    components::Transform,
    ecs::{AsAny, Entity, Id, World},
};
use cgmath::Vector2;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub trait ColliderShape: AsAny {
    fn intersecting(
        &mut self,
        parent: &(Id, Rc<RefCell<Entity>>),
        transform: &mut Transform,
        other: &(
            (Id, Rc<RefCell<Entity>>),
            ((Id, Rc<RefCell<dyn AsAny>>), (Id, Rc<RefCell<dyn AsAny>>)),
        ),
        world: &mut World,
        delta: Duration,
    ) -> bool;

    fn to_points(&mut self, _: &mut Transform) -> Option<Vec<Vector2<f32>>> {
        None
    }
}
