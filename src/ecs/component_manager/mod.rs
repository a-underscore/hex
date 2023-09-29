pub mod as_any;
pub mod component;

pub use as_any::AsAny;
pub use component::Component;

use super::{id, EntityManager, Id};
use std::{any::TypeId, collections::HashMap};

#[derive(Default)]
pub struct ComponentManager {
    free: Vec<Id>,
    cache: HashMap<Id, Box<dyn AsAny>>,
}

impl ComponentManager {
    pub fn add_gen(
        &mut self,
        eid: Id,
        cid: TypeId,
        component: Box<dyn AsAny>,
        em: &mut EntityManager,
    ) -> Option<Id> {
        let id = id::next(&mut self.free, &self.cache);

        em.entities.get_mut(&eid)?.insert(cid);
        em.components.insert((eid, cid), id);

        self.cache.insert(id, component);

        Some(id)
    }

    pub fn add<C>(&mut self, eid: Id, component: C, em: &mut EntityManager) -> Option<Id>
    where
        C: Component,
    {
        self.add_gen(eid, TypeId::of::<C>(), Box::new(component), em)
    }

    pub(super) fn rm_cache(&mut self, id: Id) {
        if self.cache.remove(&id).is_some() {
            self.free.push(id);
        }
    }

    pub fn rm_gen(&mut self, eid: Id, cid: TypeId, em: &mut EntityManager) {
        if let Some(id) = em.components.remove(&(eid, cid)) {
            self.rm_cache(id);

            if let Some(components) = em.entities.get_mut(&eid) {
                components.remove(&cid);
            }
        }
    }

    pub fn rm<C>(&mut self, eid: Id, em: &mut EntityManager)
    where
        C: Component,
    {
        self.rm_gen(eid, TypeId::of::<C>(), em);
    }

    pub fn get_gen(&self, eid: Id, cid: TypeId, em: &EntityManager) -> Option<&dyn AsAny> {
        self.get_gen_id(eid, cid, em)
            .and_then(|id| self.get_cache_gen(id))
    }

    pub fn get<C>(&self, eid: Id, em: &EntityManager) -> Option<&C>
    where
        C: Component,
    {
        self.get_gen(eid, TypeId::of::<C>(), em)
            .and_then(Self::cast)
    }

    pub fn get_gen_mut(
        &mut self,
        eid: Id,
        cid: TypeId,
        em: &EntityManager,
    ) -> Option<&mut dyn AsAny> {
        self.get_gen_id(eid, cid, em)
            .and_then(|id| self.get_cache_gen_mut(id))
    }

    pub fn get_mut<C>(&mut self, eid: Id, em: &EntityManager) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_gen_mut(eid, TypeId::of::<C>(), em)
            .and_then(Self::cast_mut)
    }

    pub fn get_gen_id(&self, eid: Id, cid: TypeId, em: &EntityManager) -> Option<Id> {
        em.components.get(&(eid, cid)).copied()
    }

    pub fn get_id<C>(&self, eid: Id, em: &EntityManager) -> Option<Id>
    where
        C: Component,
    {
        self.get_gen_id(eid, TypeId::of::<C>(), em)
    }

    pub fn get_cache_gen(&self, id: Id) -> Option<&dyn AsAny> {
        self.cache.get(&id).map(|c| c.as_ref())
    }

    pub fn get_cache<C>(&self, id: Id) -> Option<&C>
    where
        C: Component,
    {
        self.get_cache_gen(id).and_then(Self::cast)
    }

    pub fn get_cache_gen_mut(&mut self, id: Id) -> Option<&mut dyn AsAny> {
        self.cache.get_mut(&id).map(|c| c.as_mut())
    }

    pub fn get_cache_mut<C>(&mut self, id: Id) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_cache_gen_mut(id).and_then(Self::cast_mut)
    }

    pub fn cast<C>(a: &dyn AsAny) -> Option<&C>
    where
        C: Component,
    {
        a.as_any().downcast_ref()
    }

    pub fn cast_mut<C>(a: &mut dyn AsAny) -> Option<&mut C>
    where
        C: Component,
    {
        a.as_any_mut().downcast_mut()
    }
}
