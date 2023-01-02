pub mod as_any;
pub mod component;

pub use as_any::AsAny;
pub use component::Component;

use crate::ecs::{cast, cast_mut, entity_manager::EntityManager};
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
        entity_manager: &mut EntityManager,
    ) -> Option<usize> {
        let id = self
            .cache
            .keys()
            .cloned()
            .enumerate()
            .find(|(i, id)| *i != *id)
            .map(|(_, id)| id - 1)
            .unwrap_or(self.cache.len());

        entity_manager.get_mut(eid)?.insert(cid, id);

        self.cache.insert(id, component);

        Some(id)
    }

    pub fn add<C>(
        &mut self,
        eid: usize,
        component: C,
        entity_manager: &mut EntityManager,
    ) -> Option<usize>
    where
        C: Component + 'a,
    {
        self.add_gen(eid, C::id(), Box::new(component), entity_manager)
    }

    pub fn rm_gen(&mut self, eid: usize, cid: usize, entity_manager: &mut EntityManager) {
        if let Some(c) = entity_manager.get_mut(eid).and_then(|c| c.remove(&cid)) {
            self.cache.remove(&c);
        }
    }

    pub fn rm<C>(&mut self, eid: usize, entity_manager: &mut EntityManager)
    where
        C: Component,
    {
        self.rm_gen(eid, C::id(), entity_manager);
    }

    pub fn get_gen(
        &self,
        eid: usize,
        cid: usize,
        entity_manager: &EntityManager,
    ) -> Option<&dyn AsAny<'a>> {
        self.get_cached_val_gen(eid, cid, entity_manager)
            .and_then(|cid| self.get_gen_cached(cid))
    }

    pub fn get<C>(&self, eid: usize, entity_manager: &EntityManager) -> Option<&C>
    where
        C: Component,
    {
        self.get_gen(eid, C::id(), entity_manager).map(cast)
    }

    pub fn get_gen_mut(
        &mut self,
        eid: usize,
        cid: usize,
        entity_manager: &EntityManager,
    ) -> Option<&mut dyn AsAny<'a>> {
        self.get_cached_val_gen(eid, cid, entity_manager)
            .and_then(|cid| self.get_gen_cached_mut(cid))
    }

    pub fn get_mut<C>(&mut self, eid: usize, entity_manager: &EntityManager) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_gen_mut(eid, C::id(), entity_manager).map(cast_mut)
    }

    pub fn get_cached_val_gen(
        &self,
        eid: usize,
        cid: usize,
        entity_manager: &EntityManager,
    ) -> Option<usize> {
        entity_manager.get(eid)?.get(&cid).copied()
    }

    pub fn get_cached_val<C>(&self, eid: usize, entity_manager: &EntityManager) -> Option<usize>
    where
        C: Component,
    {
        self.get_cached_val_gen(eid, C::id(), entity_manager)
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
