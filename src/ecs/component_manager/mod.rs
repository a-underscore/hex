pub mod component;
pub mod generic;

pub use component::Component;
pub use generic::Generic;

use super::{id, EntityManager, Id};
use std::{collections::HashMap, mem};

#[derive(Default)]
pub struct ComponentManager<'a> {
    free: Vec<Id>,
    pub cache: HashMap<Id, (Id, Box<dyn Generic<'a>>)>,
}

impl<'a> ComponentManager<'a> {
    pub fn add_gen(
        &mut self,
        eid: Id,
        cid: Id,
        component: Box<dyn Generic<'a>>,
        em: &mut EntityManager,
    ) -> Option<Id> {
        let id = id::next(&self.cache, &mut self.free);

        em.get_mut(eid)?.insert(cid, id);

        self.cache.insert(id, (cid, component));

        Some(id)
    }

    pub fn add<C>(&mut self, eid: Id, component: C, em: &mut EntityManager) -> Option<Id>
    where
        C: Component + 'a,
    {
        self.add_gen(eid, C::id(), Box::new(component), em)
    }

    pub fn rm_gen(&mut self, eid: Id, cid: Id, em: &mut EntityManager) {
        if let Some(c) = em.get_mut(eid).and_then(|c| c.remove(&cid)) {
            self.free.push(c);
            self.cache.remove(&c);
        }
    }

    pub fn rm<C>(&mut self, eid: Id, em: &mut EntityManager)
    where
        C: Component,
    {
        self.rm_gen(eid, C::id(), em);
    }

    pub fn get_gen(&self, eid: Id, cid: Id, em: &EntityManager) -> Option<(Id, &dyn Generic<'a>)> {
        self.get_gen_id(eid, cid, em)
            .and_then(|cid| self.get_gen_cache(cid))
    }

    pub fn get<C>(&self, eid: Id, em: &EntityManager) -> Option<&C>
    where
        C: Component,
    {
        self.get_gen(eid, C::id(), em).and_then(Self::cast)
    }

    pub fn get_gen_mut(
        &mut self,
        eid: Id,
        cid: Id,
        em: &EntityManager,
    ) -> Option<(Id, &mut dyn Generic<'a>)> {
        self.get_gen_id(eid, cid, em)
            .and_then(|cid| self.get_gen_cache_mut(cid))
    }

    pub fn get_mut<C>(&mut self, eid: Id, em: &EntityManager) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_gen_mut(eid, C::id(), em).and_then(Self::cast_mut)
    }

    pub fn get_gen_id(&self, eid: Id, cid: Id, em: &EntityManager) -> Option<Id> {
        em.get(eid)?.get(&cid).copied()
    }

    pub fn get_id<C>(&self, eid: Id, em: &EntityManager) -> Option<Id>
    where
        C: Component,
    {
        self.get_gen_id(eid, C::id(), em)
    }

    pub fn get_gen_cache(&self, cid: Id) -> Option<(Id, &dyn Generic<'a>)> {
        self.cache.get(&cid).map(|(id, c)| (*id, c.as_ref()))
    }

    pub fn get_cache<C>(&self, cid: Id) -> Option<&C>
    where
        C: Component,
    {
        self.get_gen_cache(cid).and_then(Self::cast)
    }

    pub fn get_gen_cache_mut(&mut self, cid: Id) -> Option<(Id, &mut dyn Generic<'a>)> {
        self.cache.get_mut(&cid).map(|(id, c)| (*id, c.as_mut()))
    }

    pub fn get_cache_mut<C>(&mut self, cid: Id) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_gen_cache_mut(cid).and_then(Self::cast_mut)
    }

    pub fn cast<'b, C>((id, f): (Id, &'b dyn Generic<'a>)) -> Option<&'b C>
    where
        C: Component,
    {
        Some(*(C::id() == id).then(|| unsafe { mem::transmute::<&&_, &&_>(&f) })?)
    }

    pub fn cast_mut<'b, C>((id, mut f): (Id, &'b mut dyn Generic<'a>)) -> Option<&'b mut C>
    where
        C: Component,
    {
        Some(
            *(C::id() == id)
                .then(|| unsafe { mem::transmute::<&mut &mut _, &mut &mut _>(&mut f) })?,
        )
    }
}
