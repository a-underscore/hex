pub mod component_manager;

pub use component_manager::ComponentManager;

use crate::Id;
use component_manager::AsAny;
use parking_lot::RwLock;
use std::{
    any::TypeId,
    collections::{
        hash_map::{Entry, Iter},
        HashMap, HashSet,
    },
    iter::FilterMap,
    sync::Arc,
};

pub type FilteredEntities<'a> = FilterMap<
    Iter<'a, Id, (bool, HashSet<TypeId>)>,
    for<'b, 'c> fn((&'b Id, &'c (bool, HashSet<TypeId>))) -> Option<Id>,
>;

pub struct EntityManager {
    free: Vec<Id>,
    pub(crate) entities: HashMap<Id, (bool, HashSet<TypeId>)>,
    pub(crate) components: HashMap<TypeId, Box<dyn AsAny>>,
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

        self.entities.insert(id, (active, HashSet::new()));

        id
    }

    pub fn rm(&mut self, eid: Id) {
        if let Some(e) = self.entities.remove(&eid) {
            self.free.push(eid);

            let (_, components) = e;

            for cid in components {
                self.remove_component_generic(eid, cid);
            }
        }
    }

    pub fn is_active(&mut self, eid: Id) -> Option<bool> {
        self.entities.get(&eid).map(|(a, _)| *a)
    }

    pub fn set_active(&mut self, eid: Id, active: bool) {
        if let Some((a, _)) = self.entities.get_mut(&eid) {
            *a = active;
        }
    }

    pub fn get(&self, eid: Id) -> Option<&(bool, HashSet<TypeId>)> {
        self.entities.get(&eid)
    }

    pub fn add_component<C: Send + Sync + 'static>(&mut self, eid: Id, component: Arc<RwLock<C>>) {
        let entry = self
            .components
            .entry(TypeId::of::<C>())
            .or_insert(ComponentManager::<C>::new());

        if let Some(manager) = entry.as_any_mut().downcast_mut::<ComponentManager<C>>() {
            if let Some(e) = self.entities.get_mut(&eid).map(|(_, e)| e) {
                e.insert(TypeId::of::<C>());
                manager.components.insert(eid, component);
            }
        }
    }

    pub fn rm_component<C: Send + Sync + 'static>(&mut self, eid: Id) {
        self.remove_component_generic(eid, TypeId::of::<C>());
    }

    pub fn get_component<C: Send + Sync + 'static>(&self, eid: Id) -> Option<Arc<RwLock<C>>> {
        self.entities
            .get(&eid)
            .filter(|(_, e)| e.contains(&TypeId::of::<C>()))?;
        self.components
            .get(&TypeId::of::<C>())
            .and_then(|e| e.as_any().downcast_ref::<ComponentManager<C>>())
            .and_then(|m| m.components.get(&eid).cloned())
    }

    pub fn entities(&self) -> FilteredEntities {
        self.entities
            .iter()
            .filter_map(|(e, (a, _))| a.then_some(*e))
    }

    fn remove_component_generic(&mut self, eid: Id, cid: TypeId) {
        let entry = self.components.entry(cid);

        if let Entry::Occupied(mut manager) = entry {
            if manager.get_mut().remove(eid) {
                manager.remove();
            }
        }
    }
}
