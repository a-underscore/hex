use crate::{ComponentManager, Id};
use parking_lot::RwLock;
use std::{
    any::TypeId,
    collections::{hash_map::Iter, HashMap, HashSet},
    iter::FilterMap,
    sync::Arc,
};

pub type FilteredEntities<'a> = FilterMap<
    Iter<'a, Id, (bool, HashSet<TypeId>)>,
    for<'b, 'c> fn((&'b Id, &'c (bool, HashSet<TypeId>))) -> Option<Id>,
>;

#[derive(Default)]
pub struct EntityManager {
    free: Vec<Id>,
    pub(crate) entities: HashMap<Id, (bool, HashSet<TypeId>)>,
}

impl EntityManager {
    pub fn new() -> Arc<RwLock<Self>> {
        Default::default()
    }

    pub fn add_gen(&mut self, id: Id, active: bool) {
        self.entities.insert(id, (active, HashSet::new()));
    }

    pub fn add(&mut self, active: bool) -> Id {
        let id = self.free.pop().unwrap_or(self.entities.len() as Id);

        self.add_gen(id, active);

        id
    }

    pub fn rm(&mut self, eid: Id, cm: &mut ComponentManager) {
        if let Some((_, e)) = self.entities.remove(&eid) {
            self.free.push(eid);

            for cid in e {
                cm.components.remove(&(eid, cid));
            }
        }
    }

    pub fn get(&self, eid: Id) -> Option<&HashSet<TypeId>> {
        self.entities.get(&eid).and_then(|(a, e)| a.then_some(e))
    }

    pub fn active(&mut self, eid: Id) -> Option<bool> {
        let (a, _) = self.entities.get(&eid)?;

        Some(*a)
    }

    pub fn set_active(&mut self, eid: Id, active: bool) {
        if let Some((a, _)) = self.entities.get_mut(&eid) {
            *a = active;
        }
    }

    pub fn activate(&mut self, eid: Id) {
        self.set_active(eid, true);
    }

    pub fn deactivate(&mut self, eid: Id) {
        self.set_active(eid, false);
    }

    pub fn entities(&self) -> FilteredEntities<'_> {
        self.entities
            .iter()
            .filter_map(|(e, (a, _))| a.then_some(*e))
    }
}
