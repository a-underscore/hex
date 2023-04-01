pub mod component;
pub mod generic;

pub use component::Component;
pub use generic::Generic;

use super::EntityManager;
use std::{collections::BTreeMap, mem};

#[derive(Default)]
pub struct ComponentManager<'a> {
    pub cache: BTreeMap<usize, (usize, Box<dyn Generic<'a>>)>,
}

impl<'a> ComponentManager<'a> {
    pub fn add_gen(
        &mut self,
        eid: usize,
        cid: usize,
        component: Box<dyn Generic<'a>>,
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

        self.cache.insert(id, (cid, component));

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

    pub fn get_gen(
        &self,
        eid: usize,
        cid: usize,
        em: &EntityManager,
    ) -> Option<(usize, &dyn Generic<'a>)> {
        self.get_gen_cache_id(eid, cid, em)
            .and_then(|cid| self.get_gen_cache(cid))
    }

    pub fn get<C>(&self, eid: usize, em: &EntityManager) -> Option<&C>
    where
        C: Component,
    {
        self.get_gen(eid, C::id(), em).and_then(Self::cast)
    }

    pub fn get_gen_mut(
        &mut self,
        eid: usize,
        cid: usize,
        em: &EntityManager,
    ) -> Option<(usize, &mut dyn Generic<'a>)> {
        self.get_gen_cache_id(eid, cid, em)
            .and_then(|cid| self.get_gen_cache_mut(cid))
    }

    pub fn get_mut<C>(&mut self, eid: usize, em: &EntityManager) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_gen_mut(eid, C::id(), em).and_then(Self::cast_mut)
    }

    pub fn get_gen_cache_id(&self, eid: usize, cid: usize, em: &EntityManager) -> Option<usize> {
        em.get(eid)?.get(&cid).copied()
    }

    pub fn get_cache_id<C>(&self, eid: usize, em: &EntityManager) -> Option<usize>
    where
        C: Component,
    {
        self.get_gen_cache_id(eid, C::id(), em)
    }

    pub fn get_gen_cache(&self, cid: usize) -> Option<(usize, &dyn Generic<'a>)> {
        self.cache.get(&cid).map(|(id, c)| (*id, c.as_ref()))
    }

    pub fn get_cache<C>(&self, cid: usize) -> Option<&C>
    where
        C: Component,
    {
        self.get_gen_cache(cid).and_then(Self::cast)
    }

    pub fn get_gen_cache_mut(&mut self, cid: usize) -> Option<(usize, &mut dyn Generic<'a>)> {
        self.cache.get_mut(&cid).map(|(id, c)| (*id, c.as_mut()))
    }

    pub fn get_cache_mut<C>(&mut self, cid: usize) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_gen_cache_mut(cid).and_then(Self::cast_mut)
    }

    pub fn cast<'b, C>((id, f): (usize, &'b dyn Generic<'a>)) -> Option<&'b C>
    where
        C: Component,
    {
        Some(*(C::id() == id).then(|| unsafe { mem::transmute::<&&_, &&_>(&f) })?)
    }

    pub fn cast_mut<'b, C>((id, mut f): (usize, &'b mut dyn Generic<'a>)) -> Option<&'b mut C>
    where
        C: Component,
    {
        Some(
            *(C::id() == id)
                .then(|| unsafe { mem::transmute::<&mut &mut _, &mut &mut _>(&mut f) })?,
        )
    }
}
