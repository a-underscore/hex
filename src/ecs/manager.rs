use super::{cast_mut, cast_ref, Component, Components};
use std::collections::HashMap;

#[derive(Default)]
pub struct Manager<'a> {
    pub components: HashMap<usize, Components<'a>>,
}

impl<'a> Manager<'a> {
    pub fn add_c<C>(&mut self, eid: usize, component: C)
    where
        C: Component + 'a,
    {
        self.components
            .get_mut(&eid)
            .map(|c| c.insert(C::id(), (C::id(), Box::new(component))));
    }

    pub fn rm_c<C>(&mut self, eid: usize)
    where
        C: Component,
    {
        self.components.get_mut(&eid).map(|c| c.remove(&C::id()));
    }

    pub fn get_c<C>(&self, eid: usize) -> Option<&C>
    where
        C: Component,
    {
        self.components
            .get(&eid)?
            .get(&C::id())
            .map(|(_, c)| cast_ref(c))
    }

    pub fn get_c_mut<C>(&mut self, eid: usize) -> Option<&mut C>
    where
        C: Component,
    {
        self.components
            .get_mut(&eid)?
            .get_mut(&C::id())
            .map(|(_, c)| cast_mut(c))
    }

    pub fn add_e(&mut self, eid: usize) {
        self.components.insert(eid, HashMap::new());
    }

    pub fn get_e<'b>(&'b self, eid: usize) -> Option<&'b Components<'a>>
    where
        'a: 'b,
    {
        self.components.get(&eid)
    }

    pub fn get_e_mut<'b>(&'b mut self, eid: usize) -> Option<&'b mut Components<'a>>
    where
        'a: 'b,
    {
        self.components.get_mut(&eid)
    }

    pub fn rm_e(&mut self, eid: usize) {
        self.components.remove(&eid);
    }

    pub fn entities(&self) -> Vec<usize> {
        self.components.keys().cloned().collect()
    }
}
