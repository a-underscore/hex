pub mod component_manager;

pub use component_manager::ComponentManager;

use crate::Id;
use component_manager::AsAny;
use parking_lot::RwLock;
use std::{
    any::TypeId,
    collections::{
        hash_map::{Entry, Iter},
        HashMap,
    },
    iter::FilterMap,
    sync::Arc,
};

pub type FilteredEntities<'a> =
    FilterMap<Iter<'a, Id, bool>, for<'b, 'c> fn((&'b Id, &'c bool)) -> Option<Id>>;

pub struct EntityManager {
    free: Vec<Id>,
    entities: HashMap<Id, bool>,
    components: HashMap<TypeId, Box<dyn AsAny>>,
}

impl EntityManager {
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            free: Default::default(),
            entities: Default::default(),
            components: Default::default(),
        }))
    }

    pub fn add(&mut self, active: bool) -> Id {
        let id = self.free.pop().unwrap_or(self.entities.len() as Id);

        self.entities.insert(id, active);

        id
    }

    pub fn rm(&mut self, eid: Id) {
        if self.entities.remove(&eid).is_some() {
            self.free.push(eid);

            for c in self.components.values_mut() {
                c.remove(eid);
            }
        }
    }

    pub fn is_active(&self, eid: Id) -> Option<bool> {
        self.entities.get(&eid).cloned()
    }

    pub fn add_component<C: Send + Sync + 'static>(&mut self, eid: Id, component: Arc<RwLock<C>>) {
        let entry = self
            .components
            .entry(TypeId::of::<C>())
            .or_insert(ComponentManager::<C>::new());

        if let Some(manager) = entry.as_any_mut().downcast_mut::<ComponentManager<C>>() {
            if self.entities.contains_key(&eid) {
                manager.components.insert(eid, component);
            }
        }
    }

    pub fn rm_component<C: Send + Sync + 'static>(&mut self, eid: Id) {
        self.remove_component_generic(eid, TypeId::of::<C>());
    }

    pub fn get_component<C: Send + Sync + 'static>(&self, eid: Id) -> Option<Arc<RwLock<C>>> {
        self.get_component_manager::<C>()?.get(eid)
    }

    fn remove_component_generic(&mut self, eid: Id, cid: TypeId) {
        let entry = self.components.entry(cid);

        if let Entry::Occupied(mut manager) = entry {
            if manager.get_mut().remove(eid) {
                manager.remove();
            }
        }
    }

    pub fn get_component_manager<C: Send + Sync + 'static>(&self) -> Option<&ComponentManager<C>> {
        self.components
            .get(&TypeId::of::<C>())?
            .as_any()
            .downcast_ref::<ComponentManager<C>>()
    }

    pub fn entities(&self) -> FilteredEntities {
        self.entities.iter().filter_map(|(e, a)| a.then_some(*e))
    }
}
