pub mod component_manager;

pub use component_manager::ComponentManager;

use crate::Id;
use component_manager::AsAny;
use parking_lot::RwLock;
use std::{
    any::TypeId,
    collections::{
        hash_map::{Entry, Keys},
        HashMap, HashSet,
    },
    iter::Cloned,
    sync::Arc,
};

#[derive(Default)]
pub struct EntityManager {
    free: Vec<Id>,
    pub(crate) entities: HashMap<Id, HashSet<TypeId>>,
    pub(crate) components: HashMap<TypeId, Box<dyn AsAny>>,
}

impl EntityManager {
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            free: Default::default(),
            entities: Default::default(),
            components: HashMap::new(),
        }))
    }

    pub fn add(&mut self) -> Id {
        let id = self.free.pop().unwrap_or(self.entities.len() as Id);

        self.entities.insert(id, HashSet::new());

        id
    }

    pub fn rm(&mut self, eid: Id) {
        if let Some(e) = self.entities.remove(&eid) {
            self.free.push(eid);

            for cid in e {
                self.remove_component_generic(eid, cid);
            }
        }
    }

    pub fn add_component<C: Send + Sync + 'static>(&mut self, eid: Id, component: Arc<RwLock<C>>) {
        let entry = self
            .components
            .entry(TypeId::of::<C>())
            .or_insert(Box::new(ComponentManager::<C>::new()));

        if let Some(manager) = entry
            .as_any()
            .downcast_ref::<Arc<RwLock<ComponentManager<C>>>>()
        {
            manager.write().components.insert(eid, component);
        }
    }

    pub fn rm_component<C: Send + Sync + 'static>(&mut self, eid: Id) {
        self.remove_component_generic(eid, TypeId::of::<C>());
    }

    pub fn get_component<C: Send + Sync + 'static>(&self, eid: Id) -> Option<Arc<RwLock<C>>> {
        self.entities
            .get(&eid)
            .filter(|e| e.contains(&TypeId::of::<C>()))?;

        self.components
            .get(&TypeId::of::<C>())
            .and_then(|e| {
                e.as_any()
                    .downcast_ref::<Arc<RwLock<ComponentManager<C>>>>()
            })
            .and_then(|m| m.read().components.get(&eid).cloned())
    }

    pub fn entities(&self) -> Cloned<Keys<'_, Id, HashSet<TypeId>>> {
        self.entities.keys().cloned()
    }

    fn remove_component_generic(&mut self, eid: Id, cid: TypeId) {
        let entry = self.components.entry(cid);

        if let Entry::Occupied(manager) = entry {
            if manager.get().remove(eid) {
                manager.remove();
            }
        }
    }
}
