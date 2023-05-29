pub mod component;
pub mod generic;

pub use component::Component;
pub use generic::Generic;

use super::{id, EntityManager, Id};
use std::{collections::HashMap, mem};

#[derive(Default)]
pub struct ComponentManager<'a> {
    free: Vec<Id>,
    pub cache: HashMap<(Id, Id), Box<dyn Generic<'a>>>,
}

impl<'a> ComponentManager<'a> {
    pub fn add_gen(
        &mut self,
        eid: Id,
        cid: Id,
        component: Box<dyn Generic<'a>>,
        em: &mut EntityManager,
    ) -> Option<Id> {
        let id = id::next(&mut self.free, &self.cache);

        em.entities.get_mut(&eid)?.insert(cid, id);

        self.cache.insert((cid, id), component);

        Some(id)
    }

    pub fn add<C>(&mut self, eid: Id, component: C, em: &mut EntityManager) -> Option<Id>
    where
        C: Component + 'a,
    {
        self.add_gen(eid, C::id(), Box::new(component), em)
    }

    pub fn rm_gen(&mut self, eid: Id, cid: Id, em: &mut EntityManager) {
        if let Some(id) = em.entities.get_mut(&eid).and_then(|c| c.remove(&cid)) {
            self.free.push(id);
            self.cache.remove(&(cid, id));
        }
    }

    pub fn rm<C>(&mut self, eid: Id, em: &mut EntityManager)
    where
        C: Component,
    {
        self.rm_gen(eid, C::id(), em);
    }

    pub fn get_gen(&self, eid: Id, cid: Id, em: &EntityManager) -> Option<&dyn Generic<'a>> {
        self.get_gen_id(eid, cid, em)
            .and_then(|id| self.get_cache_gen(cid, id))
    }

    pub fn get<C>(&self, eid: Id, em: &EntityManager) -> Option<&C>
    where
        C: Component,
    {
        self.get_gen(eid, C::id(), em).map(Self::cast)
    }

    pub fn get_gen_mut(
        &mut self,
        eid: Id,
        cid: Id,
        em: &EntityManager,
    ) -> Option<&mut dyn Generic<'a>> {
        self.get_gen_id(eid, cid, em)
            .and_then(|id| self.get_cache_gen_mut(cid, id))
    }

    pub fn get_mut<C>(&mut self, eid: Id, em: &EntityManager) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_gen_mut(eid, C::id(), em).map(Self::cast_mut)
    }

    pub fn get_gen_id(&self, eid: Id, cid: Id, em: &EntityManager) -> Option<Id> {
        em.entities.get(&eid)?.get(&cid).copied()
    }

    pub fn get_id<C>(&self, eid: Id, em: &EntityManager) -> Option<Id>
    where
        C: Component,
    {
        self.get_gen_id(eid, C::id(), em)
    }

    pub fn get_cache_gen(&self, cid: Id, id: Id) -> Option<&dyn Generic<'a>> {
        self.cache.get(&(cid, id)).map(|c| c.as_ref())
    }

    pub fn get_cache<C>(&self, id: Id) -> Option<&C>
    where
        C: Component,
    {
        self.get_cache_gen(C::id(), id).map(Self::cast)
    }

    pub fn get_cache_gen_mut(&mut self, cid: Id, id: Id) -> Option<&mut dyn Generic<'a>> {
        self.cache.get_mut(&(cid, id)).map(|c| c.as_mut())
    }

    pub fn get_cache_mut<C>(&mut self, cid: Id) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_cache_gen_mut(C::id(), cid).map(Self::cast_mut)
    }

    pub fn cast<'b, C>(f: &'b dyn Generic<'a>) -> &'b C
    where
        C: Component,
    {
        unsafe { mem::transmute::<&&_, &&_>(&f) }
    }

    pub fn cast_mut<'b, C>(mut f: &'b mut dyn Generic<'a>) -> &'b mut C
    where
        C: Component,
    {
        unsafe { mem::transmute::<&mut &mut _, &mut &mut _>(&mut f) }
    }
}
