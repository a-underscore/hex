use super::{cast_mut, cast_ref, AsAny, Component, EntityManager};
use std::collections::HashMap;

#[derive(Default)]
pub struct ComponentManager<'a> {
    pub cache: HashMap<usize, Box<dyn AsAny<'a>>>,
    pub(super) freed: Vec<usize>,
}

impl<'a> ComponentManager<'a> {
    pub fn add_gen(
        &mut self,
        entity_manager: &mut EntityManager,
        eid: usize,
        cid: usize,
        component: Box<dyn AsAny<'a>>,
    ) -> Option<usize> {
        let id = self.freed.pop().unwrap_or(self.cache.len());

        entity_manager
            .entities
            .get_mut(&eid)
            .map(|c| c.insert(cid, id))?;
        self.cache.insert(id, component);

        Some(id)
    }

    pub fn add<C>(
        &mut self,
        entity_manager: &mut EntityManager,
        id: usize,
        component: C,
    ) -> Option<usize>
    where
        C: Component + 'a,
    {
        self.add_gen(entity_manager, id, C::id(), Box::new(component))
    }

    pub fn rm_gen(&mut self, entity_manager: &mut EntityManager, id: usize, cid: usize) {
        if let Some(c) = entity_manager
            .entities
            .get_mut(&id)
            .and_then(|c| c.remove(&cid))
        {
            self.cache.remove(&c);
            self.freed.push(c);
        }
    }

    pub fn rm_c<C>(&mut self, entity_manager: &mut EntityManager, id: usize)
    where
        C: Component,
    {
        self.rm_gen(entity_manager, id, C::id());
    }

    pub fn get_gen(
        &self,
        entity_manager: &EntityManager,
        id: usize,
        cid: usize,
    ) -> Option<&dyn AsAny<'a>> {
        entity_manager
            .entities
            .get(&id)
            .and_then(|c| c.get(&cid).copied())
            .and_then(|cid| self.get_gen_cached(cid))
    }

    pub fn get<C>(&self, entity_manager: &EntityManager, id: usize) -> Option<&C>
    where
        C: Component,
    {
        self.get_gen(entity_manager, id, C::id()).map(cast_ref)
    }

    pub fn get_gen_mut(
        &mut self,
        entity_manager: &mut EntityManager,
        id: usize,
        cid: usize,
    ) -> Option<&mut dyn AsAny<'a>> {
        entity_manager
            .entities
            .get(&id)
            .and_then(|c| c.get(&cid).copied())
            .and_then(|cid| self.get_gen_cached_mut(cid))
    }

    pub fn get_mut<C>(&mut self, entity_manager: &mut EntityManager, id: usize) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_gen_mut(entity_manager, id, C::id())
            .map(|c| cast_mut(c))
    }

    pub fn get_gen_cached(&self, cid: usize) -> Option<&dyn AsAny<'a>> {
        self.cache.get(&cid).map(|c| c.as_ref())
    }

    pub fn get_cached<C>(&self, cid: usize) -> Option<&C>
    where
        C: Component,
    {
        self.get_gen_cached(cid).map(cast_ref)
    }

    pub fn get_gen_cached_mut(&mut self, cid: usize) -> Option<&mut dyn AsAny<'a>> {
        self.cache.get_mut(&cid).map(|c| c.as_mut())
    }

    pub fn get_cached_mut<C>(&mut self, cid: usize) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_gen_cached_mut(cid).map(|c| cast_mut(c))
    }
}
