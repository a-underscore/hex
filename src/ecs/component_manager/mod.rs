pub mod as_any;
pub mod component;

pub use as_any::AsAny;
pub use component::Component;

use super::{cast, cast_mut, entity_manager::EntityManager};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct ComponentManager<'a> {
    pub cache: BTreeMap<usize, Box<dyn AsAny<'a>>>,
}

impl<'a> ComponentManager<'a> {
    pub fn add_gen(
        &mut self,
        eid: usize,
        cid: usize,
        component: Box<dyn AsAny<'a>>,
        em: &mut EntityManager,
    ) -> Option<usize> {
        let id = self
            .cache
            .keys()
            .cloned()
            .enumerate()
            .find(|(i, id)| *i != *id)
            .map(|(_, id)| id - 1)
            .unwrap_or(self.cache.len());

        em.get_mut(eid)?.insert(cid, id);

        self.cache.insert(id, component);

        Some(id)
    }

    pub fn add<C>(&mut self, eid: usize, component: C, em: &mut EntityManager) -> Option<usize>
    where
        C: Component + 'a,
    {
        self.add_gen(eid, C::id(), Box::new(component), em)
    }

    pub fn rm_gen(&mut self, eid: usize, cid: usize, em: &mut EntityManager) {
        if let Some(c) = em.get_mut(eid).and_then(|c| c.remove(&cid)) {
            self.cache.remove(&c);
        }
    }

    pub fn rm<C>(&mut self, eid: usize, em: &mut EntityManager)
    where
        C: Component,
    {
        self.rm_gen(eid, C::id(), em);
    }

    pub fn get_gen(&self, eid: usize, cid: usize, em: &EntityManager) -> Option<&dyn AsAny<'a>> {
        self.get_gen_cached_id(eid, cid, em)
            .and_then(|cid| self.get_gen_cached(cid))
    }

    pub fn get<C>(&self, eid: usize, em: &EntityManager) -> Option<&C>
    where
        C: Component,
    {
        self.get_gen(eid, C::id(), em).map(cast)
    }

    pub fn get_gen_mut(
        &mut self,
        eid: usize,
        cid: usize,
        em: &EntityManager,
    ) -> Option<&mut dyn AsAny<'a>> {
        self.get_gen_cached_id(eid, cid, em)
            .and_then(|cid| self.get_gen_cached_mut(cid))
    }

    pub fn get_mut<C>(&mut self, eid: usize, em: &EntityManager) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_gen_mut(eid, C::id(), em).map(cast_mut)
    }

    pub fn get_gen_cached_id(&self, eid: usize, cid: usize, em: &EntityManager) -> Option<usize> {
        em.get(eid)?.get(&cid).copied()
    }

    pub fn get_cached_id<C>(&self, eid: usize, em: &EntityManager) -> Option<usize>
    where
        C: Component,
    {
        self.get_gen_cached_id(eid, C::id(), em)
    }

    pub fn get_gen_cached(&self, cid: usize) -> Option<&dyn AsAny<'a>> {
        self.cache.get(&cid).map(Box::as_ref)
    }

    pub fn get_cached<C>(&self, cid: usize) -> Option<&C>
    where
        C: Component,
    {
        self.get_gen_cached(cid).map(cast)
    }

    pub fn get_gen_cached_mut(&mut self, cid: usize) -> Option<&mut dyn AsAny<'a>> {
        self.cache.get_mut(&cid).map(Box::as_mut)
    }

    pub fn get_cached_mut<C>(&mut self, cid: usize) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_gen_cached_mut(cid).map(cast_mut)
    }
}
