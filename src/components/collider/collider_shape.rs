use crate::{
    components::Transform,
    ecs::{AsAny, Entity, Id, World},
};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub trait ColliderShape: AsAny {
    fn get_intersecting(
        &mut self,
        world: &mut World,
        parent: &(Id, Rc<RefCell<Entity>>),
        transform: &Transform,
        components: &Vec<(
            (Id, Rc<RefCell<Entity>>),
            ((Id, Rc<RefCell<dyn AsAny>>), (Id, Rc<RefCell<dyn AsAny>>)),
        )>,
        delta: Duration,
    ) -> Vec<(Id, Rc<RefCell<Entity>>)>;
}
