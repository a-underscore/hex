use super::{cast_mut, cast_ref, Component, AsAny};
use std::collections::HashMap;

#[derive(Default)]
pub struct ComponentManager<'a> {
    pub components: HashMap<usize, usize>,
    pub cache: Vec<Box<dyn AsAny<'a>>>,
}

impl<'a> ComponentManager<'a> {
    pub fn add<C>(&mut self, eid: usize, component: C) -> Option<()>
    where
        C: Component + 'a,
    {
        let cid = self.components.len();

        self.components.insert(eid, cid);

        self.cache.push(Box::new(component));

        Some(())
    }

    pub fn rm<C>(&mut self)
    where
        C: Component,
    {
        self.components.remove(&C::id()).map(|ci|
            self.cache.remove(ci));
    }

    pub fn get<C>(&self) -> Option<&C>
    where
        C: Component,
    {
        self.components
            .get(&C::id())
            .and_then(|c| Some(cast_ref(self.cache.get(*c)?)))
    }

    pub fn get_mut<C>(&mut self) -> Option<&mut C>
    where
        C: Component,
    {
        self.components
            .get_mut(&C::id())
            .and_then(|c| Some(cast_mut(self.cache.get_mut(*c)?)))
    }
}
