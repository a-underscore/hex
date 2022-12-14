use super::{cast_mut, cast_ref, Component, Components, Entities};
use std::collections::HashMap;

#[derive(Default)]
pub struct Manager {
    pub entities: Entities,
}

impl Manager {
    pub fn add_c<C>(&mut self, eid: usize, component: C)
    where
        C: Component + 'static,
    {
        self.entities
            .get_mut(&eid)
            .map(|c| c.insert(C::id(), (C::id(), Box::new(component))));
    }

    pub fn rm_c<C>(&mut self, eid: usize)
    where
        C: Component,
    {
        self.entities.get_mut(&eid).map(|c| c.remove(&C::id()));
    }

    pub fn get_c<C>(&self, eid: usize) -> Option<&C>
    where
        C: Component,
    {
        self.entities
            .get(&eid)?
            .get(&C::id())
            .map(|(_, c)| cast_ref(c))
    }

    pub fn get_c_mut<C>(&mut self, eid: usize) -> Option<&mut C>
    where
        C: Component,
    {
        self.entities
            .get_mut(&eid)?
            .get_mut(&C::id())
            .map(|(_, c)| cast_mut(c))
    }

    pub fn add_e(&mut self, eid: usize) {
        self.entities.insert(eid, HashMap::new());
    }

    pub fn get_e(&self, eid: usize) -> Option<&Components> {
        self.entities.get(&eid)
    }

    pub fn get_e_mut(&mut self, eid: usize) -> Option<&mut Components> {
        self.entities.get_mut(&eid)
    }

    pub fn rm_e(&mut self, eid: usize) {
        self.entities.remove(&eid);
    }
}
